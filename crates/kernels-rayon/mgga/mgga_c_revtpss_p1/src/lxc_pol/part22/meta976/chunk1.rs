//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3285/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3285(t62282: f64, t1522: f64, t49880: f64, t50878: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t61310: f64, t61311: f64, t61313: f64, t61316: f64, t61317: f64, t62269: f64, t62270: f64, t62273: f64, t62275: f64, t62277: f64, t62279: f64) -> (f64, f64, f64, f64) {
    let t62283 = 48.0_f64 * t62282;
    let t62285 = 8.0_f64 * t49880 * t1522;
    let t62286 = 24.0_f64 * t50878;
    let t62287 = t61310 + t61311 + t61313 + t61316 - t61317 + t40067 - t40072 + t62269 + t40167 - t40171 - t62270 - t40184 + t62273 + t62275 + t62277 + t62279 + t62283 + t62285 + t62286;
    (t62283, t62285, t62286, t62287)
}
