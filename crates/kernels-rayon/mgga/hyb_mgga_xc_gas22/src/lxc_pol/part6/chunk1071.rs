//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1071/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1071(t10325: f64, t699: f64, t702: f64, t10286: f64, t10290: f64, t10295: f64, t10299: f64, t10302: f64, t10306: f64, t10310: f64, t10313: f64, t10317: f64, t10322: f64, t3023: f64, t572: f64, t6278: f64, t6279: f64, t8288: f64, t8291: f64, t8293: f64, t8294: f64) -> (f64, f64) {
    let t10327 = t699 * t702 * t10325;
    let t10330 = -t6278 - 2.0_f64 / 243.0_f64 * t6279 - 4.0_f64 / 243.0_f64 * t8288 + t8291 - t8293 - 2.0_f64 / 81.0_f64 * t8294 + t10286 / 243.0_f64 - 5.0_f64 / 243.0_f64 * t572 * t10290 + 2.0_f64 / 27.0_f64 * t572 * t10295 + 4.0_f64 / 81.0_f64 * t3023 * t10299 - t10302 / 81.0_f64 - t572 * t10306 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t3023 * t10310 + t10313 / 162.0_f64 - t572 * t10317 / 81.0_f64 + t572 * t10322 / 27.0_f64 - t572 * t10327 / 54.0_f64;
    (t10327, t10330)
}
