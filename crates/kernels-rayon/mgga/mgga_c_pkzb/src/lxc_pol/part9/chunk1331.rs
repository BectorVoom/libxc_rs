//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1331/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1331(t1306: f64, t22538: f64, t22540: f64, t22542: f64, t22544: f64, t22547: f64, t22550: f64, t22553: f64, t22556: f64, t22559: f64, t23555: f64, t3286: f64, t6601: f64, t8572: f64) -> f64 {
    let t23561 = 6.0_f64 * t1306 * t23555 * t8572 - t1306 * t3286 * t6601 + t22538 + t22540 + t22542 + t22544 - t22547 - t22550 - t22553 - t22556 - t22559;
    t23561
}
