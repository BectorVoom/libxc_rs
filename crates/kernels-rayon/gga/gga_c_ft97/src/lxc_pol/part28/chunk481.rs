//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 481/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk481(t165: f64, t7340: f64, t28: f64, t1360: f64, t1389: f64, t167: f64, t2185: f64, t7312: f64, t1359: f64, t1391: f64, t574: f64, t1384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7341 = t7340 * t165;
    let t7342 = t28 * t7341;
    let t7345 = t1360 * t1389;
    let t7346 = t28 * t7345;
    let t7350 = t2185 * t167 * t7312;
    let t7354 = t574 * t1391 * t1359;
    let t7357 = t1359 * t1384;
    (t7341, t7342, t7345, t7346, t7350, t7354, t7357)
}
