//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 580/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk580(t1454: f64, t626: f64, t1453: f64, t2331: f64, t1444: f64, t2341: f64, t1449: f64, t2349: f64, t1409: f64, t2433: f64, t2440: f64, t1472: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4041 = t626 * t1454;
    let t4043 = t2331 * t1453;
    let t4049 = t2341 * t1444;
    let t4059 = t2349 * t1449;
    let t4080 = t2433 * t1409;
    let t4087 = t2440 * t1409;
    let t4100 = t1472 * t751;
    (t4041, t4043, t4049, t4059, t4080, t4087, t4100)
}
