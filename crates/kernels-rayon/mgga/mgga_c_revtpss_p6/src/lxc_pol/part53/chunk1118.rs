//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1118/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1118(t2439: f64, t785: f64, t8578: f64, t8580: f64, t121210: f64, t2453: f64, t8705: f64, t25304: f64, t32237: f64, t121142: f64, t596: f64, t8571: f64) -> (f64, f64, f64, f64, f64) {
    let t121259 = 0.4818682326780666368e-3_f64 * t2439 * t785 * t8578 * t8580;
    let t121272 = t2453 * t8705 * t121210;
    let t121273 = 0.3718732920905101082e-5_f64 * t121272;
    let t121275 = t25304 * t8705 * t121210;
    let t121276 = 0.19835721400107809171e-4_f64 * t121275;
    let t121285 = t2453 * t32237;
    let t121287 = 0.95199562775170587692e-3_f64 * t121285 * t121142;
    let t121305 = t8571 * t596;
    (t121259, t121273, t121276, t121287, t121305)
}
