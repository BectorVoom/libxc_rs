//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 636/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk636(t1043: f64, t2863: f64, t2862: f64, t392: f64, t395: f64, t1027: f64, t2834: f64, t2836: f64, t2843: f64, t2848: f64, t2852: f64, t1025: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2864 = t2863 * t1043;
    let t2866 = 2.0_f64 * t2862 * t2864;
    let t2868 = 1.0_f64 / t395 / t392;
    let t2869 = t1027 * t1027;
    let t2870 = t2868 * t2869;
    let t2872 = 4.0_f64 / 9.0_f64 * t2834;
    let t2877 = t2872 - 2.0_f64 / 9.0_f64 * t2836 - 2.0_f64 / 9.0_f64 * t2843 + 2.0_f64 / 3.0_f64 * t2848 + t2852 / 3.0_f64;
    let t2878 = t1025 * t2877;
    (t2864, t2866, t2868, t2869, t2870, t2872, t2877, t2878)
}
