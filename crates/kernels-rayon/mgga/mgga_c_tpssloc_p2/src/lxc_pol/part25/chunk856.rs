//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 856/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk856(t1060: f64, t11077: f64, t11023: f64, t3201: f64, t1003: f64, t10359: f64, t1058: f64, t1061: f64, t1063: f64, t11024: f64, t11028: f64, t11031: f64, t11034: f64, t11037: f64, t11040: f64, t11043: f64, t11046: f64, t11049: f64, t11051: f64, t11055: f64, t11059: f64, t11061: f64, t11065: f64, t11067: f64, t3076: f64, t3180: f64, t3186: f64, t3189: f64, t3193: f64, t3197: f64, t3200: f64, t3202: f64, t3204: f64, t353: f64, t384: f64) -> f64 {
    let t11078 = t11077 * t1060;
    let t11081 = t11023 * t3201;
    let t11084 = 3.0_f64 * t3180 * t3197 + 6.0_f64 * t3186 * t11024 + t1058 * t11028 + 3.0_f64 * t1058 * t11031 + 6.0_f64 * t11034 * t3189 - 3.0_f64 * t11037 * t3202 - 3.0_f64 * t3200 * t11040 + t353 * t11043 + t11046 * t11049 + 3.0_f64 * t11051 * t1061 + 6.0_f64 * t3186 * t11055 + 6.0_f64 * t11059 * t11061 - 6.0_f64 * t11065 * t11067 + 3.0_f64 * t1003 * t3204 + 3.0_f64 * t3076 * t1063 + t10359 * t384 + 6.0_f64 * t3180 * t3193 + 3.0_f64 * t1058 * t11078 - 3.0_f64 * t3200 * t11081;
    t11084
}
