//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3328/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3328(t18392: f64, t262: f64, t11084: f64, t18860: f64, t2430: f64, t4541: f64, t51780: f64, t5966: f64, t5970: f64, t62275: f64, t62277: f64, t62279: f64, t62283: f64, t62285: f64, t62286: f64, t62290: f64, t62293: f64, t62296: f64, t775: f64) -> f64 {
    let t63146 = t262 * t18392;
    let t63158 = -6.0_f64 * t11084 * t4541 * t5966 + 6.0_f64 * t18860 * t2430 * t4541 + 12.0_f64 * t4541 * t63146 * t775 + 12.0_f64 * t51780 * t5970 + t62275 + t62277 + t62279 + t62283 + t62285 + t62286 + t62290 + t62293 + t62296;
    t63158
}
