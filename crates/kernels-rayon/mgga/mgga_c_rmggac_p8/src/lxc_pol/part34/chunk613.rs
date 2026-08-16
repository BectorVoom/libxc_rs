//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 613/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk613(t15470: f64, t739: f64, t15206: f64, t15209: f64, t15212: f64, t15215: f64, t515: f64, t9523: f64, t3352: f64, t3351: f64, t15218: f64, t15221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15471 = t739 * t15470;
    let t15472 = 0.2993560425465952141e-1_f64 * t15471;
    let t15473 = 0.87596530464506835935e-6_f64 * t15206;
    let t15474 = 0.19709219354514038085e-5_f64 * t15209;
    let t15475 = 0.87596530464506835935e-6_f64 * t15212;
    let t15476 = 0.2627895913935205078e-5_f64 * t15215;
    let t15477 = t515 * t9523;
    let t15478 = t3352 * t15477;
    let t15479 = t3351 * t15478;
    let t15480 = 0.12769379967989351819e-4_f64 * t15479;
    let t15481 = 0.85129199786595678799e-5_f64 * t15218;
    let t15482 = 0.85129199786595678799e-5_f64 * t15221;
    (t15472, t15473, t15474, t15475, t15476, t15478, t15480, t15481, t15482)
}
