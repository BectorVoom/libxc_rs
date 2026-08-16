//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 601/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk601(t2646: f64, t2652: f64, t2657: f64, t2660: f64, t2662: f64, t2665: f64, t2670: f64, t2672: f64, t2678: f64, t2682: f64, t2685: f64, t336: f64, t363: f64, t925: f64, t931: f64, t951: f64, t967: f64) -> f64 {
    let t2688 = -t925 * t2646 / 144.0_f64 - t2652 + 19.0_f64 / 1728.0_f64 * t2657 * t363 - t2660 / 432.0_f64 + 11.0_f64 / 108.0_f64 * t2662 * t336 - t2665 / 54.0_f64 - t2670 - t967 * t2672 / 2304.0_f64 + t2678 / 2304.0_f64 - t2682 * t951 / 288.0_f64 - t2685 * t931 / 54.0_f64;
    t2688
}
