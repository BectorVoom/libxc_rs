//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2716/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2716(t1266: f64, t1393: f64, t1774: f64, t19450: f64, t19451: f64, t19461: f64, t19534: f64, t20293: f64, t20347: f64, t20350: f64, t20720: f64, t2314: f64, t4034: f64, t4073: f64, t510: f64, t5107: f64, t5118: f64, t5450: f64, t5457: f64, t6468: f64, t652: f64, t75555: f64) -> f64 {
    let t75733 = -2.0_f64 * t1266 * t20347 * t652 - 6.0_f64 * t1774 * t19534 * t652 - t1266 * t20293 + t1393 * t20350 - 3.0_f64 * t1774 * t19450 - 6.0_f64 * t1774 * t19461 - 6.0_f64 * t19451 * t4073 - 2.0_f64 * t20720 * t2314 - 2.0_f64 * t20720 * t4034 - t510 * t75555 - 3.0_f64 * t5107 * t5450 - 6.0_f64 * t5107 * t5457 + 3.0_f64 * t5118 * t6468;
    t75733
}
