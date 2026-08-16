//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 906/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk906(t1017: f64, t944: f64, t136: f64, t357: f64, t7599: f64, t1074: f64, t7309: f64, t1059: f64, t2015: f64, t1062: f64, t2035: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13671 = t944 * t1017;
    let t13690 = t7599 * t136 * t357;
    let t13691 = 0.57050000000000000002e1_f64 * t13690;
    let t13693 = t7309 * t136 * t1074;
    let t13694 = 0.70633333333333333334e1_f64 * t13693;
    let t13695 = t2015 * t1059;
    let t13696 = t13695 * t1062;
    let t13698 = t2035 * t1059;
    let t13699 = t13698 * t3127;
    (t13671, t13690, t13691, t13693, t13694, t13695, t13696, t13698, t13699)
}
