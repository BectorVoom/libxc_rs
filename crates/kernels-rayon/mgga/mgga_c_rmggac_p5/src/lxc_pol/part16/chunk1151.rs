//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1151/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1151(t10288: f64, t10289: f64, t10290: f64, t10291: f64, t10292: f64, t10293: f64, t10294: f64, t10295: f64, t10296: f64, t42434: f64, t8084: f64, t10301: f64, t10302: f64, t10303: f64, t10305: f64, t10308: f64, t10311: f64, t10312: f64, t10313: f64, t42444: f64, t42445: f64, t8094: f64) -> (f64, f64) {
    let t49834 = t8084 - t42434 + t10288 - t10289 - t10290 - t10291 - t10292 + t10293 + t10294 - t10295 - t10296;
    let t49837 = t8094 + t42444 - t10301 - t10302 - t10303 - t42445 - t10305 - t10308 - t10311 + t10312 + t10313;
    (t49834, t49837)
}
