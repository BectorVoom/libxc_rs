//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1592/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1592(t11219: f64, t14726: f64, t136: f64, t4775: f64, t699: f64, t14736: f64, t3297: f64, t14740: f64, t14731: f64, t1113: f64, t14749: f64, t14753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14778 = t11219 * t14726;
    let t14779 = t136 * t14778;
    let t14781 = t699 * t4775;
    let t14782 = 0.22076e0_f64 * t14781;
    let t14783 = t3297 * t14736;
    let t14784 = t136 * t14783;
    let t14786 = t3297 * t14740;
    let t14787 = t136 * t14786;
    let t14789 = t3297 * t14731;
    let t14790 = t136 * t14789;
    let t14792 = t1113 * t14749;
    let t14793 = t136 * t14792;
    let t14795 = t1113 * t14753;
    (t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795)
}
