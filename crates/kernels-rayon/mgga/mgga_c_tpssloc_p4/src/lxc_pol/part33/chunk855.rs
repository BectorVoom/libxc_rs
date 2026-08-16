//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 855/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk855(t1496: f64, t9541: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64, t2535: f64, t1471: f64, t32: f64, t118: f64, t1474: f64, t2375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13087 = t9541 * t1496;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    let t13115 = t32 * t1471;
    let t13123 = t1474 * t118;
    let t13124 = t13123 * t2375;
    (t13087, t13107, t13109, t13113, t13115, t13124)
}
