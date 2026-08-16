//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1031/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1031(t25: f64, t514: f64, t3665: f64, t606: f64, t3704: f64, t1298: f64, t2249: f64, t9257: f64, t28: f64, t517: f64, t1081: f64, t3673: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t11985 = t25 * t25;
    let t11987 = 1.0_f64 / t514 / t11985;
    let t11988 = t3665 * t606;
    let t11991 = t3704 * t606;
    let t11997 = piecewise3(t26, 0.0_f64, 8.0_f64 / 27.0_f64 * t11987 * t11988 - 2.0_f64 / 3.0_f64 * t11991 * t2249 + 2.0_f64 / 3.0_f64 * t1298 * t9257);
    let t11998 = t28 * t28;
    let t12000 = 1.0_f64 / t517 / t11998;
    let t12001 = t3673 * t1081;
    (t11985, t11987, t11988, t11991, t11997, t11998, t12000, t12001)
}
