//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1273/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1273(t33155: f64, t6066: f64, t7630: f64, t10860: f64, t23279: f64, t33627: f64, t2033: f64, t2365: f64, t2610: f64, t8720: f64, t15349: f64, t3474: f64) -> (f64, f64, f64, f64, f64) {
    let t33640 = 0.71500979903700853338e0_f64 * t7630 * t6066 * t33155;
    let t33642 = 0.14300195980740170668e1_f64 * t23279 * t10860;
    let t33645 = 0.14300195980740170668e1_f64 * t7630 * t6066 * t33627;
    let t33648 = t2033 * t2365 * t2610 * t8720;
    let t33649 = 0.14896037479937677779e-1_f64 * t33648;
    let t33650 = t15349 * t3474;
    (t33640, t33642, t33645, t33649, t33650)
}
