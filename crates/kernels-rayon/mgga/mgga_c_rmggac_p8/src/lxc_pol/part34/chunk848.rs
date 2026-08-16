//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 848/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk848(t13975: f64, t8577: f64, t1970: f64, t1971: f64, t209: f64, t2367: f64, t476: f64, t515: f64, t14225: f64, t9152: f64, t9188: f64, t3352: f64, t9158: f64) -> (f64, f64, f64, f64) {
    let t75186 = t8577 * t13975;
    let t75192 = t1970 * t1971 * t515 * t2367 * t476 * t209;
    let t75195 = t14225 * t9188 * t9152;
    let t75198 = t14225 * t3352 * t9158;
    (t75186, t75192, t75195, t75198)
}
