//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 895/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk895(t3702: f64, t6065: f64, t2156: f64, t3698: f64, t1306: f64, t803: f64, t9336: f64, t9338: f64, t9345: f64, t9347: f64, t9350: f64, t9354: f64, t9358: f64, t9361: f64, t9363: f64, t9365: f64, t9367: f64, t9392: f64, t9394: f64, t9396: f64, t9400: f64, t9530: f64, t9535: f64, t9537: f64) -> (f64, f64, f64) {
    let t9721 = t3702 * t6065;
    let t9725 = t3698 * t2156;
    let t9728 = 2.0_f64 * t1306 * t803 * t9721 - t1306 * t803 * t9725 + t9336 + t9338 - t9345 - t9347 + t9350 - t9354 + t9358 - t9361 + t9363 - t9365 + t9367 + t9392 + t9394 - t9396 + t9400 + t9530 - t9535 + t9537;
    (t9721, t9725, t9728)
}
