//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1193/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1193(t1339: f64, t20568: f64, t6936: f64, t20501: f64, t6916: f64, t20570: f64, t6945: f64, t1361: f64, t20563: f64, t26288: f64, t20479: f64, t6952: f64) -> (f64, f64, f64, f64, f64) {
    let t107118 = t6936 * t1339 * t20568;
    let t107120 = t6916 * t20501;
    let t107123 = t6945 * t20570;
    let t107126 = t26288 * t1361 * t20563;
    let t107133 = t6952 * t20479;
    (t107118, t107120, t107123, t107126, t107133)
}
