//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 459/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk459<F: Float>(t2846: F, t221: F, t346: F, t696: F, t345: F, t1003: F, t1007: F, t360: F, t365: F, t1038: F, t72: F, t1087: F, t1066: F, t828: F, t1043: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3070 = 0.19755555555555555556e-1 * t2846;
    let t3080 = t221 * t696 * t346;
    let t3082 = t345 * t3080 / 432.0;
    let t3086 = t1003 * t1007;
    let t3088 = t360 * t365;
    let t3089 = t1038 * t72;
    let t3090 = t3088 * t3089;
    let t3091 = t1087 * t3090;
    let t3092 = t828 * t1066;
    let t3093 = t1043 * t73;
    (t3070, t3080, t3082, t3086, t3088, t3089, t3090, t3091, t3092, t3093)
}
