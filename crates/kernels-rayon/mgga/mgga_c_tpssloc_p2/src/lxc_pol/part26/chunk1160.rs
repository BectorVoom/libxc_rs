//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1160/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1160(t1089: f64, t1240: f64, t1251: f64, t607: f64, t24601: f64, t225: f64, t3590: f64, t497: f64, t462: f64, t3597: f64, t3599: f64, t7300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24602 = t1240 * t1089;
    let t24603 = t607 * t1251;
    let t24604 = t24602 * t24603;
    let t24605 = t24601 * t24604;
    let t24611 = t3590 * t225 * t497;
    let t24612 = t462 * t24611;
    let t24615 = t225 * t3597;
    let t24616 = t24615 * t3599;
    let t24617 = t7300 * t24616;
    (t24602, t24604, t24605, t24611, t24612, t24615, t24616, t24617)
}
