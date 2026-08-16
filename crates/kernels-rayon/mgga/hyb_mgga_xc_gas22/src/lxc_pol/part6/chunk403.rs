//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 403/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk403(t1161: f64, t1555: f64, t1117: f64, t1134: f64, t1144: f64, t1149: f64, t1158: f64, t1167: f64, t1169: f64, t1172: f64, t1536: f64, t1540: f64, t1543: f64, t1546: f64, t1549: f64, t1552: f64, t510: f64, t518: f64) -> (f64, f64) {
    let t1556 = t1161 * t1555;
    let t1563 = 2.0_f64 * t1117 * t1536 - 2.0_f64 * t510 * t1540 + 6.0_f64 * t1134 * t1543 - 6.0_f64 * t518 * t1546 + 3.0_f64 * t1144 * t1549 - 3.0_f64 * t1149 * t1552 - 4.0_f64 / 9.0_f64 * t1158 * t1556 + t1167 * t1549 - t1169 * t1552 - 4.0_f64 / 9.0_f64 * t1172 * t1556;
    (t1556, t1563)
}
