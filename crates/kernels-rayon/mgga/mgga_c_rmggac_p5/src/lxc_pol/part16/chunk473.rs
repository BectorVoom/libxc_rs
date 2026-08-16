//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 473/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk473(t1944: f64, t321: f64, t333: f64, t171: f64, t1811: f64, t433: f64, t498: f64, t5389: f64, t4085: f64, t4112: f64, t4114: f64, t1425: f64, t4056: f64, t4062: f64, t4064: f64, t4074: f64, t4077: f64, t4080: f64, t4083: f64, t4089: f64, t4101: f64, t4106: f64, t4111: f64, t5375: f64, t5376: f64, t5377: f64, t5382: f64, t5395: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5963 = t1944 * t321;
    let t5966 = t1944 * t333;
    let t5969 = t1811 * t171;
    let t5970 = t5969 * t433;
    let t5971 = 0.5848223622634646207e0_f64 * t5970;
    let t5974 = t5389 * t498;
    let t5977 = 0.10843581300301739842e-1_f64 * t4085;
    let t5978 = 32.0_f64 * t4112;
    let t5979 = 20.0_f64 * t4114;
    let t5980 = -t4056 + t4062 + t4064 + t5375 - t5376 - t4074 - t5971 + 0.373092e0_f64 * t5395 * t5377 - 0.186546e0_f64 * t1425 * t5974 - t4077 - t4080 + t4083 + t5977 + t4089 - t4101 + t4106 + t4111 + t5978 + t5979 - t5382;
    (t5963, t5966, t5971, t5977, t5978, t5979, t5980)
}
