//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 543/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk543(t1307: f64, t1610: f64, t4440: f64, t1444: f64, t617: f64, t2642: f64, t1600: f64, t1601: f64, t2645: f64, t1606: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4441 = t1307 * t1610;
    let t4442 = t4440 * t4441;
    let t4445 = t617 * t1444;
    let t4446 = t4445 * t2642;
    let t4447 = t1600 * t4446;
    let t4450 = t1601 * t2645;
    let t4451 = t1600 * t4450;
    let t4455 = 1.0_f64 / t1606 / t616;
    (t4441, t4442, t4446, t4447, t4450, t4451, t4455)
}
