//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 611/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk611(t15471: f64, t15209: f64, t15215: f64, t515: f64, t9523: f64, t3352: f64, t3351: f64, t15218: f64, t15221: f64, t15228: f64, t15232: f64, t15236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15472 = 0.2993560425465952141e-1_f64 * t15471;
    let t15474 = 0.19709219354514038085e-5_f64 * t15209;
    let t15476 = 0.2627895913935205078e-5_f64 * t15215;
    let t15477 = t515 * t9523;
    let t15478 = t3352 * t15477;
    let t15479 = t3351 * t15478;
    let t15480 = 0.12769379967989351819e-4_f64 * t15479;
    let t15481 = 0.85129199786595678799e-5_f64 * t15218;
    let t15482 = 0.85129199786595678799e-5_f64 * t15221;
    let t15485 = 0.15961724959986689775e-4_f64 * t15228;
    let t15486 = 0.1276937996798935182e-4_f64 * t15232;
    let t15487 = 0.2553875993597870364e-4_f64 * t15236;
    (t15472, t15474, t15476, t15478, t15480, t15481, t15482, t15485, t15486, t15487)
}
