//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1258/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1258(t37455: f64, t39074: f64, t40388: f64, t40391: f64, t40411: f64, t41316: f64, t41319: f64, t41322: f64, t41329: f64, t41332: f64, t41335: f64, t41339: f64, t41342: f64, t41346: f64, t41350: f64) -> f64 {
    let t42204 = t41316 - t41319 - 0.76845137554657911361e-2_f64 * t37455 - t41322 - 0.72042316457491791901e-3_f64 * t40388 - 0.1440846329149835838e-2_f64 * t40391 + t41329 - t41332 - t41335 - t41339 - t41342 - 0.1440846329149835838e-2_f64 * t40411 - t41346 + t41350 + t39074;
    t42204
}
