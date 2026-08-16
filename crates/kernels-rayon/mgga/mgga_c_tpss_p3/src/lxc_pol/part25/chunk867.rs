//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 867/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk867(t720: f64, t7870: f64, t2209: f64, t712: f64, t177: f64, t185: f64, t2213: f64, t7813: f64, t705: f64, t7850: f64, t169: f64, t2271: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7871 = t7870 * t720;
    let t7875 = 1.0_f64 / t2209 / t712;
    let t7876 = t177 * t7875;
    let t7878 = 1.0_f64 / t2213 / t185;
    let t7879 = t7813 * t7878;
    let t7882 = t7850 * t705;
    let t7886 = 1.0_f64 / t2271 / t169;
    (t7871, t7875, t7876, t7878, t7879, t7882, t7886)
}
