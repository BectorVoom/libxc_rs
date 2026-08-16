//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1753/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1753(t23477: f64, t23479: f64, t6721: f64, t6739: f64, t6741: f64, t344: f64, t6729: f64, t6740: f64, t3103: f64, t6755: f64, t3034: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23480 = t23477 * t23479;
    let t23482 = t6721 * t6739;
    let t23483 = t23482 * t6741;
    let t23488 = t6729 * t344;
    let t23489 = t6740 * t23488;
    let t23500 = t6755 * t3103;
    let t23508 = 1.0_f64 / t3034 / t371;
    (t23480, t23482, t23483, t23488, t23489, t23500, t23508)
}
