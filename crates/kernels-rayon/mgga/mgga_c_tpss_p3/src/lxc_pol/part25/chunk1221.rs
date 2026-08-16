//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1221/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1221(t1165: f64, t13133: f64, t1338: f64, t13554: f64, t1799: f64, t18898: f64, t19305: f64, t19656: f64, t20288: f64, t20289: f64, t20294: f64, t20319: f64, t2056: f64, t3493: f64, t3537: f64, t4347: f64, t5801: f64, t5815: f64, t6234: f64, t6323: f64, t645: f64) -> f64 {
    let t20322 = 2.0_f64 * t1165 * t20319 + 2.0_f64 * t13133 * t1799 + 2.0_f64 * t1338 * t18898 + 2.0_f64 * t1338 * t20294 + 2.0_f64 * t13554 * t1799 + 2.0_f64 * t1799 * t19305 + 2.0_f64 * t1799 * t19656 + 2.0_f64 * t20289 * t645 + 2.0_f64 * t2056 * t6323 + 2.0_f64 * t3493 * t5815 + 2.0_f64 * t3537 * t5801 + 2.0_f64 * t4347 * t6323 + 2.0_f64 * t5815 * t6234 + t20288;
    t20322
}
