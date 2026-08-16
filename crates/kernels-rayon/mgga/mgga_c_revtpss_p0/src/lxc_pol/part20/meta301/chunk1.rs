//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1186/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1186(t12378: f64, t448: f64, t300: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64) {
    let t12379 = t12378 * t448;
    let t12381 = 0.19751673498613801407e-1_f64 * t300 * t12379;
    let t12382 = 0.55403703703703703703e-1_f64 * t12295;
    let t12393 = -t12382 + 0.23744444444444444444e-1_f64 * t12297 + 0.11872222222222222222e-1_f64 * t12299 - 0.35616666666666666666e-1_f64 * t12301 - 0.17808333333333333333e-1_f64 * t12303 + 0.19787037037037037037e-1_f64 * t12307 - 0.71233333333333333332e-1_f64 * t12310 - 0.35616666666666666666e-1_f64 * t12292 + 0.10685e0_f64 * t12314 + 0.10685e0_f64 * t12317 + 0.17808333333333333333e-1_f64 * t12320;
    (t12379, t12381, t12393)
}
