//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1158/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1158(t122317: f64, t32705: f64, t120996: f64, t122282: f64, t7286: f64, t786: f64, t2453: f64, t25946: f64, t32715: f64, t122277: f64, t25898: f64, t25901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122319 = 0.19039912555034117539e-1_f64 * t32705 * t122317;
    let t122321 = 0.7052700942260554372e-3_f64 * t120996;
    let t122327 = t786 * t122282 * t7286;
    let t122331 = 0.3427046870806409921e-2_f64 * t2453 * t32715 * t25946;
    let t122335 = t122277 * t25898;
    let t122336 = t122335 * t25901;
    (t122319, t122321, t122327, t122331, t122335, t122336)
}
