//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1680/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1680(t1842: f64, t7213: f64, t3887: f64, t1807: f64, t7191: f64, t1375: f64, t16460: f64, t1843: f64, t2092: f64, t22908: f64, t22910: f64, t22922: f64, t22928: f64, t22941: f64, t24082: f64, t24156: f64, t24157: f64, t5215: f64, t5321: f64, t5354: f64, t568: f64, t7194: f64, t7199: f64, t7214: f64) -> (f64, f64, f64) {
    let t27131 = t7213 * t1842;
    let t27132 = t3887 * t27131;
    let t27137 = t1807 * t7191;
    let t27141 = -t5215 * t7214 + t22908 + t22910 - t16460 * t2092 - t7194 * t5354 + t22922 + 2.0_f64 * t1375 * t27132 - t24082 * t1843 + t24156 + t24157 - 0.82246703342411321825e-2_f64 * t22928 + t27137 * t568 + 2.0_f64 * t5321 * t7199 - t22941;
    (t27132, t27137, t27141)
}
