//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1375/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1375<F: Float>(t33873: F, t9851: F, t2736: F, t79: F, t84689: F, t33870: F, t34968: F, t4419: F, t9516: F, t109539: F, t109704: F, t109707: F, t32433: F, t33802: F, t33851: F, t34945: F, t34969: F, t35026: F, t9512: F, t9519: F, t9544: F, t9869: F) -> (F, F) {
    let t120174 = t9851 * t33873;
    let t120183 = t84689 * t79 * t2736;
    let t120188 = t9851 * t33870;
    let t120194 = t4419 * t34968;
    let t120195 = t9516 * t120194;
    let t120201 = 0.11574074074074074074e-2 * t109539 + 0.34722222222222222223e-2 * t120174 - 0.27777777777777777779e-1 * t33851 * t9869 + 0.52083333333333333333e-2 * t35026 * t9544 + 0.52083333333333333333e-2 * t35026 * t9519 + 0.20104166666666666667e-2 * t120183 * t9519 - 0.27777777777777777779e-1 * t33802 * t9869 + 0.34722222222222222223e-2 * t120188 - 0.116403125e-2 * t109704 * t34945 - 0.116403125e-2 * t109707 * t34945 + 0.6701388888888888889e-3 * t120195 - 0.53611111111111111112e-2 * t32433 * t34969 + 0.52083333333333333333e-2 * t9512 * t34969;
    (t120194, t120201)
}
