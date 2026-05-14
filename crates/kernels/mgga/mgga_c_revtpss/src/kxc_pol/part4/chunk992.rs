//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 992/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk992<F: Float>(t12901: F, t3600: F, t11262: F, t1251: F, t1247: F, t3704: F, t3708: F, t1284: F, t3566: F, t3624: F, t126: F, t482: F, t828: F) -> (F, F, F, F, F, F) {
    let t12902 = t3600 * t12901;
    let t12904 = t11262 * t1251;
    let t12905 = t1247 * t12904;
    let t12907 = t3708 * t3704;
    let t12909 = t3566 * t1284;
    let t12910 = t12909 * t3624;
    let t12915 = t126 * t482;
    let t12916 = t828 * t12915;
    (t12902, t12905, t12907, t12910, t12915, t12916)
}
