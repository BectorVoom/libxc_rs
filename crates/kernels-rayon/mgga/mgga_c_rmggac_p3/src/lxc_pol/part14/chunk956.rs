//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 956/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk956(t1971: f64, t511: f64, t615: f64, t7230: f64, t848: f64, t34847: f64, t8843: f64, t1525: f64, t352: f64, t515: f64, t866: f64, t2320: f64, t34878: f64) -> (f64, f64, f64, f64, f64) {
    let t40389 = t7230 * t1971 * t511 * t615 * t848;
    let t40391 = t34847 * t8843;
    let t40396 = t7230 * t1971 * t515 * t1525 * t352;
    let t40401 = t7230 * t1971 * t515 * t615 * t866;
    let t40403 = t34878 * t2320;
    (t40389, t40391, t40396, t40401, t40403)
}
