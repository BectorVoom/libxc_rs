//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1114/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1114(t121184: f64, t31805: f64, t32240: f64, t1385: f64, t46361: f64, t2470: f64, t32239: f64, t32238: f64, t1955: f64, t2681: f64, t8571: f64, t8575: f64) -> (f64, f64, f64, f64, f64) {
    let t121185 = t31805 * t121184;
    let t121186 = t121185 * t32240;
    let t121188 = t46361 * t1385;
    let t121197 = t32239 * t2470;
    let t121199 = 0.19039912555034117539e-1_f64 * t32238 * t121197;
    let t121202 = t1955 * t8571 * t2681 * t8575;
    (t121186, t121188, t121197, t121199, t121202)
}
