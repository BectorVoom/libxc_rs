//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1190/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1190(t24745: f64, t7363: f64, t3502: f64, t491: f64, t24813: f64, t1209: f64, t1090: f64, t7376: f64, t221: f64, t4899: f64, t2127: f64, t2135: f64, t477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27454 = t24745 * t7363;
    let t27488 = t3502 * t491;
    let t27489 = t24813 * t27488;
    let t27495 = t1209 * t491;
    let t27496 = t24813 * t27495;
    let t27532 = t7376 * t1090;
    let t27548 = t221 * t4899;
    let t27549 = t2127 * t27548;
    let t27550 = t2135 * t477;
    (t27454, t27489, t27495, t27496, t27532, t27549, t27550)
}
