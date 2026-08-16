//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 884/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk884(t638: f64, t7292: f64, t8475: f64, t289: f64, t39290: f64, t39293: f64, t39296: f64, t39297: f64, t39301: f64, t39306: f64, t39308: f64, t39310: f64, t39312: f64, t39314: f64, t39316: f64, t39319: f64, t39320: f64, t39323: f64, t39325: f64, t39330: f64) -> f64 {
    let t39333 = t638 * t7292 * t8475;
    let t39335 = -t39290 - 0.25538759935978703639e-4_f64 * t39293 + t39296 - 0.42564599893297839398e-5_f64 * t39297 + 0.11971293719990017331e-4_f64 * t39301 + 0.53205749866622299248e-5_f64 * t39306 - 0.33105799917009430643e-4_f64 * t39308 - 0.42564599893297839398e-5_f64 * t39310 + 0.1064114997332445985e-4_f64 * t39312 - 0.31923449919973379548e-4_f64 * t39314 + 0.31923449919973379548e-4_f64 * t39316 + t39319 - 0.4726e1_f64 * t289 * t39320 + 0.85129199786595678796e-5_f64 * t39323 - 0.85129199786595678796e-5_f64 * t39325 + 0.1064114997332445985e-4_f64 * t39330 + 0.81300399444200075504e-3_f64 * t39333;
    t39335
}
