//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2234/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2234(t16830: f64, t25255: f64, t25262: f64, t2617: f64, t28413: f64, t4234: f64, t4291: f64, t5585: f64, t812: f64, t81679: f64, t829: f64, t87154: f64, t92516: f64, t98461: f64, t98464: f64, t98467: f64, t98471: f64, t98475: f64, t98478: f64, t98482: f64, t98486: f64, t98488: f64, t98490: f64, t98494: f64) -> f64 {
    let t98497 = 2.0_f64 * t2617 * t28413 + 2.0_f64 * t812 * t81679 * t5585 - 2.0_f64 * t812 * t25255 * t4234 + 0.3289868133696452873e-1_f64 * t98461 + 0.3289868133696452873e-1_f64 * t98464 + 0.16449340668482264365e-1_f64 * t98467 - t87154 + t92516 + 0.3289868133696452873e-1_f64 * t98471 - 0.3289868133696452873e-1_f64 * t98475 + 0.3289868133696452873e-1_f64 * t98478 - 0.16449340668482264365e-1_f64 * t98482 + 0.16449340668482264365e-1_f64 * t98486 + 0.19190897446562641759e-1_f64 * t98488 - 0.38381794893125283518e-1_f64 * t98490 - 2.0_f64 * t16830 * t25262 - t4291 * t98494 * t829;
    t98497
}
