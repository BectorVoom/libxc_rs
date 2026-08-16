//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 973/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk973(t1851: f64, t4621: f64, t15239: f64, t11081: f64, t6762: f64, t3514: f64, t1262: f64, t1662: f64, t11072: f64, t330: f64, t6774: f64, t829: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20563 = t4621 * t1851;
    let t20564 = t15239 * t20563;
    let t20569 = t11081 * t6762;
    let t20570 = t3514 * t20569;
    let t20572 = t1851 * t1262;
    let t20573 = t1662 * t20572;
    let t20574 = t11072 * t20573;
    let t20578 = t6774 * t330;
    let t20579 = t20578 * t829;
    (t20564, t20570, t20572, t20573, t20574, t20579)
}
