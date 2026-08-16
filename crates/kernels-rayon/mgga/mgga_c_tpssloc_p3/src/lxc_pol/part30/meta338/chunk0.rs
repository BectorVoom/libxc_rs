//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1370/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1370(t225: f64, t4143: f64, t4145: f64, t1496: f64, t9541: f64, t2427: f64, t4101: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64, t2535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13053 = t4143 * t225;
    let t13065 = t4145 * t225;
    let t13087 = t9541 * t1496;
    let t13105 = 8.0_f64 * t2427 * t4101;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    (t13053, t13065, t13087, t13105, t13107, t13109, t13113)
}
