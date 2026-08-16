//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 492/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk492(t213: f64, t2551: f64, t2653: f64, t2740: f64, t2820: f64, t2014: f64, t978: f64, t211: f64, t215: f64, t414: f64, t690: f64, t2026: f64, t982: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t2822 = t2551 + t2653 + t2740 + t2820;
    let t2828 = t2014 * t978;
    let t2831 = t215 * t211;
    let t2835 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t2828 * t690 + 8.0_f64 / 3.0_f64 * t2831 * t414);
    let t2836 = t2026 * t982;
    (t2822, t2835, t2836)
}
