//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 494/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk494(t4118: f64, t1022: f64, t4056: f64, t4062: f64, t4064: f64, t4074: f64, t4077: f64, t4080: f64, t4083: f64, t4089: f64, t4101: f64, t4106: f64, t4108: f64, t4111: f64, t4585: f64, t5372: f64, t5375: f64, t5376: f64, t5377: f64, t5380: f64, t5381: f64, t5382: f64) -> (f64, f64) {
    let t5383 = 24.0_f64 * t4118;
    let t5384 = -t4056 + t4062 + 0.186546e0_f64 * t1022 * t5372 - t4064 + t5375 + t5376 - t4074 + 0.373092e0_f64 * t4585 * t5377 - t4077 - t4080 + t4083 + t5380 + t4089 - t4101 + t4106 + t4108 + t4111 + t5381 - t5382 - t5383;
    (t5383, t5384)
}
