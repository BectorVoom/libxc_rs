//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 965/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk965<F: Float>(t27928: F, t303: F, t1768: F, t26796: F, t1250: F, t14570: F, t2173: F, t2175: F, t26685: F, t26728: F, t26745: F, t26748: F, t26751: F, t26758: F, t26787: F, t26814: F, t26826: F, t27895: F, t27904: F, t27911: F, t27915: F, t27919: F, t27926: F, t7687: F, t7693: F, t7703: F, t7711: F, t8030: F, t8034: F, t8038: F, t8042: F) -> (F, F, F, F, F) {
    let t27929 = t303 * t27928;
    let t27931 = t26796 * t1768;
    let t27932 = t303 * t27931;
    let t27936 = t14570 * t1250;
    let t27939 = 0.69505208333333333333e-3 * t8030 * t7711 + 0.69505208333333333333e-3 * t8030 * t7693 + 0.92754700520833333333e-4 * t27895 * t7693 + 0.92754700520833333333e-4 * t26728 * t8034 + 0.61782407407407407408e-3 * t26745 - 0.23168402777777777778e-3 * t26748 * t8038 - 0.46336805555555555556e-3 * t7703 * t27904 + 0.11054629629629629629e-2 * t26751 - 0.7722800925925925926e-4 * t26758 + 0.11054629629629629629e-2 * t26787 - 0.92754700520833333333e-4 * t26685 * t27911 + 0.69505208333333333333e-3 * t2173 * t27915 + 0.69505208333333333333e-3 * t2173 * t27919 + 0.69505208333333333333e-3 * t7687 * t8042 - 0.24872916666666666666e-2 * t27926 + 0.16581944444444444444e-2 * t27929 - 0.24872916666666666666e-2 * t27932 + 0.30918233506944444445e-4 * t26814 + 0.16581944444444444444e-2 * t26826 - 0.69505208333333333333e-3 * t27936 * t2175;
    (t27929, t27931, t27932, t27936, t27939)
}
