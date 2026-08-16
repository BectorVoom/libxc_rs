//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1378/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1378(t106956: f64, t1873: f64, t19451: f64, t7467: f64, t106878: f64, t106881: f64, t106921: f64, t106923: f64, t106932: f64, t106934: f64, t106937: f64, t106939: f64, t106941: f64, t106953: f64, t1458: f64, t20347: f64, t24999: f64, t33085: f64, t5493: f64, t6517: f64, t96686: f64) -> f64 {
    let t106958 = 6.0_f64 * t106956 * t1873;
    let t106960 = 6.0_f64 * t19451 * t7467;
    let t106961 = 6.0_f64 * t1458 * t96686 + 2.0_f64 * t20347 * t6517 + 6.0_f64 * t24999 * t5493 + 6.0_f64 * t33085 * t5493 + t106878 + 6.0_f64 * t106881 + t106921 + t106923 + t106932 + t106934 + t106937 + t106939 + t106941 + t106953 + t106958 + t106960;
    t106961
}
