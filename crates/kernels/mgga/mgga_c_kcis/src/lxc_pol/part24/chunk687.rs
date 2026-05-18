//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 687/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk687<F: Float>(t7718: F, t8047: F, t1020: F, t1748: F, t2179: F, t303: F, t1768: F, t7726: F, t1774: F, t356: F, t2173: F, t2175: F, t7690: F, t7701: F, t7703: F, t7717: F, t8030: F, t8034: F, t8038: F, t8042: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8048 = t7718 * t8047;
    let t8049 = t1020 * t8048;
    let t8051 = t1748 * t2179;
    let t8052 = t303 * t8051;
    let t8054 = t7726 * t1768;
    let t8055 = t303 * t8054;
    let t8057 = t356 * t1774;
    let t8058 = t303 * t8057;
    let t8060 = -F::new(0.69505208333333333333e-3) * t8030 * t2175 + F::new(0.92754700520833333333e-4) * t7690 * t8034 - t7701 - F::new(0.23168402777777777778e-3) * t7703 * t8038 + F::new(0.69505208333333333333e-3) * t2173 * t8042 + F::new(0.69505208333333333333e-3) * t2173 * t8034 + t7717 + F::new(0.16581944444444444444e-2) * t8049 + F::new(0.24872916666666666666e-2) * t8052 - F::new(0.24872916666666666666e-2) * t8055 + F::new(0.16581944444444444444e-2) * t8058;
    (t8048, t8049, t8051, t8052, t8054, t8055, t8057, t8058, t8060)
}
