//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 928/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk928(t1971: f64, t515: f64, t615: f64, t7230: f64, t866: f64, t2320: f64, t34878: f64, t1525: f64, t209: f64, t236: f64, t476: f64, t7453: f64) -> (f64, f64, f64) {
    let t40401 = t7230 * t1971 * t515 * t615 * t866;
    let t40403 = t34878 * t2320;
    let t40414 = t7453 * t1971 * t236 * t1525 * t476 * t209;
    (t40401, t40403, t40414)
}
