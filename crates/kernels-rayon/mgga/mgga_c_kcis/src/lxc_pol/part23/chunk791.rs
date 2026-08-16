//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 791/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk791(t12147: f64, t3985: f64, t1368: f64, t3970: f64, t3990: f64, t1376: f64, t1370: f64, t3999: f64, t1377: f64, t3978: f64, t1444: f64, t451: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12148 = t12147 * t3985;
    let t12149 = t1368 * t12148;
    let t12151 = t3970 * t3990;
    let t12152 = t1368 * t12151;
    let t12158 = t1376 * t1376;
    let t12159 = 1.0_f64 / t12158;
    let t12185 = t1370 * t3999;
    let t12194 = t3978 * t1377;
    let t12216 = 1.0_f64 / t451 / t1444;
    (t12149, t12152, t12159, t12185, t12194, t12216)
}
