//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 855/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk855(t13004: f64, t205: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64, t4134: f64, t9546: f64, t1496: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13005 = t205 * t13004;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    let t13022 = t9546 * t4134;
    let t13087 = t9541 * t1496;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    (t13005, t13010, t13012, t13022, t13087, t13107, t13109)
}
