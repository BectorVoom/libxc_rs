//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1287/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1287(t1185: f64, t2197: f64, t6193: f64, t18617: f64, t6143: f64, t8205: f64, t22511: f64, t22515: f64, t22517: f64, t22519: f64, t22522: f64, t22526: f64, t22528: f64, t22530: f64, t22532: f64, t22534: f64, t22536: f64, t22538: f64, t22540: f64, t22542: f64, t22544: f64, t22547: f64, t22550: f64, t22553: f64) -> (f64, f64, f64) {
    let t22556 = 2.0_f64 * t2197 * t1185 * t6193;
    let t22559 = 0.62071215503128080361e4_f64 * t18617 * t8205 * t6143;
    let t22560 = -t22511 + t22515 - t22517 - t22519 - t22522 - t22526 - t22528 - t22530 - t22532 - t22534 - t22536 + t22538 + t22540 + t22542 + t22544 - t22547 - t22550 - t22553 - t22556 - t22559;
    (t22556, t22559, t22560)
}
