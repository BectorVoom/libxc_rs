//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 779/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk779(t549: f64, t875: f64, t2643: f64, t7468: f64, t2640: f64, t2668: f64, t2675: f64, t2678: f64, t2680: f64, t7417: f64, t7421: f64, t7424: f64, t7427: f64, t7430: f64, t7436: f64, t7439: f64, t7441: f64, t7443: f64, t7447: f64, t7449: f64, t7453: f64, t7457: f64, t7461: f64, t7464: f64, t878: f64) -> (f64, f64, f64) {
    let t7469 = t549 * t875;
    let t7470 = t7469 * t2643;
    let t7471 = t7468 * t7470;
    let t7472 = t2640 * t7471;
    let t7474 = -0.23666877659387696117e-1_f64 * t7417 + 0.35973654042269298099e1_f64 * t7421 * t878 - 0.75734008510040627576e0_f64 * t7424 - 0.1465164556873572827e3_f64 * t7427 * t2675 + 0.73258227843678641352e2_f64 * t7430 * t2680 + 0.18314556960919660338e2_f64 * t7436 - 0.91572784804598301689e1_f64 * t7439 + 11.0_f64 / 108.0_f64 * t7441 + t7443 / 54.0_f64 + t7447 - 0.91572784804598301689e1_f64 * t7449 * t7453 + 0.27471835441379490507e2_f64 * t2668 * t7457 + 0.71000632978163088351e-1_f64 * t2640 * t7461 - 0.13735917720689745254e2_f64 * t2678 * t7464 + 0.94667510637550784468e-1_f64 * t7472;
    (t7470, t7471, t7474)
}
