//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 500/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk500<F: Float>(t5: F, t577: F, t946: F, t1009: F, t578: F, t1012: F, t1015: F, t1528: F, t195: F, t4056: F, t4062: F, t4064: F, t4074: F, t4077: F, t4080: F, t4083: F, t4089: F, t4101: F, t4106: F, t4108: F, t4111: F, t5375: F, t5376: F, t5380: F, t5381: F) -> (F, F, F, F, F, F) {
    let t5443 = t577 * t5;
    let t5444 = t5443 * t946;
    let t5445 = F::cast_from(0.10843581300301739842e-1_f64) * t5444;
    let t5446 = t1009 * t578;
    let t5447 = F::new(20.0) * t5446;
    let t5448 = t1012 * t578;
    let t5449 = F::new(12.0) * t5448;
    let t5450 = t1015 * t578;
    let t5451 = F::new(32.0) * t5450;
    let t5452 = t195 * t1528;
    let t5455 = -t4056 + t4062 - t4064 + t5375 + t5376 - t4074 - t4077 - t4080 + t4083 + t5380 + t4089 - t4101 + t4106 + t4108 + t4111 + t5381;
    (t5445, t5447, t5449, t5451, t5452, t5455)
}
