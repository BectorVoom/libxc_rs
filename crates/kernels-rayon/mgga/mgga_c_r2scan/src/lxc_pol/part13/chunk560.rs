//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 560/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk560(t2214: f64, t923: f64, t514: f64, t1604: f64, t2605: f64, t788: f64, t938: f64, t2201: f64, t785: f64, t910: f64, t2207: f64, t780: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2682 = t2214 * t923;
    let t2683 = t514 * t2682;
    let t2685 = t1604 * t2605;
    let t2687 = t788 * t938;
    let t2689 = t2201 * t785 * t2687;
    let t2691 = t788 * t910;
    let t2693 = t2207 * t785 * t2691;
    let t2696 = t980 * t780;
    (t2682, t2683, t2685, t2687, t2689, t2691, t2693, t2696)
}
