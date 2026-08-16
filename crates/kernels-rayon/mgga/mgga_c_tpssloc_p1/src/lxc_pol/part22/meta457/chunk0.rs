//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1828/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1828(t112: f64, t20292: f64, t1441: f64, t5456: f64, t1453: f64, t5464: f64, t9365: f64, t4043: f64, t5488: f64, t1444: f64, t5468: f64, t9384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20293 = t20292 * t112;
    let t20296 = t1441 * t5456;
    let t20304 = t5464 * t1453;
    let t20305 = t9365 * t20304;
    let t20308 = t4043 * t5488;
    let t20311 = t5468 * t1444;
    let t20312 = t9384 * t20311;
    (t20293, t20296, t20304, t20305, t20308, t20311, t20312)
}
