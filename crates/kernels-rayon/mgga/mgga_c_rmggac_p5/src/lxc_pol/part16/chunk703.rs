//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 703/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk703(t305: f64, t9957: f64, t793: f64, t9765: f64, t797: f64, t9999: f64, t2068: f64, t9873: f64, t7829: f64, t9889: f64, t1763: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10154 = t305 * t9957;
    let t10156 = t793 * t9765;
    let t10158 = t797 * t9999;
    let t10162 = t2068 * t9873;
    let t10164 = t7829 * t9889;
    let t10166 = t36 * t1763;
    (t10154, t10156, t10158, t10162, t10164, t10166)
}
