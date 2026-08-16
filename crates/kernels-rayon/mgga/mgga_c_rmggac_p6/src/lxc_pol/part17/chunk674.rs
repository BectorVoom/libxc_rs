//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 674/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk674(t9737: f64, t2060: f64, t6557: f64, t903: f64, t1953: f64, t71: f64, t131: f64, t638: f64, t639: f64, t2338: f64, t574: f64, t1950: f64, t640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9738 = 0.85129199786595678796e-5_f64 * t9737;
    let t9739 = t2060 * t6557;
    let t9740 = t903 * t9739;
    let t9741 = 0.8980681276397856423e-1_f64 * t9740;
    let t9745 = t71 * t1953;
    let t9746 = t9745 * t131;
    let t9748 = t638 * t639 * t9746;
    let t9749 = 0.15243824895787514157e-3_f64 * t9748;
    let t9750 = t2338 * t574;
    let t9752 = t638 * t639 * t9750;
    let t9753 = 0.30487649791575028314e-3_f64 * t9752;
    let t9754 = t640 * t1950;
    (t9738, t9739, t9741, t9745, t9746, t9749, t9750, t9753, t9754)
}
