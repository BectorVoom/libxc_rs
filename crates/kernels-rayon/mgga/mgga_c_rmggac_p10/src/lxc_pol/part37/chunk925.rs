//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 925/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk925(t76737: f64, t118: f64, t2001: f64, t618: f64, t699: f64, t7720: f64, t73851: f64, t73854: f64, t73865: f64, t73871: f64, t73873: f64, t73875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76738 = 0.85129199786595678796e-5_f64 * t76737;
    let t76741 = t2001 * t118 * t699 * t618;
    let t76742 = t7720 * t76741;
    let t76743 = 0.42564599893297839398e-5_f64 * t76742;
    let t76744 = 0.2627895913935205078e-5_f64 * t73851;
    let t76745 = 0.2627895913935205078e-5_f64 * t73854;
    let t76748 = 0.19709219354514038085e-5_f64 * t73865;
    let t76749 = 0.64054962902170623776e-5_f64 * t73871;
    let t76750 = 0.85129199786595678799e-5_f64 * t73873;
    let t76751 = 0.2553875993597870364e-4_f64 * t73875;
    (t76738, t76743, t76744, t76745, t76748, t76749, t76750, t76751)
}
