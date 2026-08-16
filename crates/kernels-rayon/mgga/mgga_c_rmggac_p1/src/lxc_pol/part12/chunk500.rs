//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 500/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk500(t5: f64, t577: f64, t946: f64, t1009: f64, t578: f64, t1012: f64, t1015: f64, t1528: f64, t195: f64, t4056: f64, t4062: f64, t4064: f64, t4074: f64, t4077: f64, t4080: f64, t4083: f64, t4089: f64, t4101: f64, t4106: f64, t4108: f64, t4111: f64, t5375: f64, t5376: f64, t5380: f64, t5381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5443 = t577 * t5;
    let t5444 = t5443 * t946;
    let t5445 = 0.10843581300301739842e-1_f64 * t5444;
    let t5446 = t1009 * t578;
    let t5447 = 20.0_f64 * t5446;
    let t5448 = t1012 * t578;
    let t5449 = 12.0_f64 * t5448;
    let t5450 = t1015 * t578;
    let t5451 = 32.0_f64 * t5450;
    let t5452 = t195 * t1528;
    let t5455 = -t4056 + t4062 - t4064 + t5375 + t5376 - t4074 - t4077 - t4080 + t4083 + t5380 + t4089 - t4101 + t4106 + t4108 + t4111 + t5381;
    (t5445, t5447, t5449, t5451, t5452, t5455)
}
