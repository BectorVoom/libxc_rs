//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 474/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk474<F: Float>(t4066: F, t4069: F, t1535: F, t446: F, t4085: F, t4114: F, t4116: F, t4118: F, t1022: F, t4056: F, t4062: F, t4064: F, t4074: F, t4077: F, t4080: F, t4083: F, t4089: F, t4101: F, t4106: F, t4108: F, t4111: F, t4585: F, t5372: F) -> (F, F, F, F, F, F, F) {
    let t5375 = 48.0 * t4066;
    let t5376 = 80.0 * t4069;
    let t5377 = t1535 * t446;
    let t5380 = 0.21687162600603479684e-1 * t4085;
    let t5381 = 40.0 * t4114;
    let t5382 = 12.0 * t4116;
    let t5383 = 24.0 * t4118;
    let t5384 = -t4056 + t4062 + 0.186546e0 * t1022 * t5372 - t4064 + t5375 + t5376 - t4074 + 0.373092e0 * t4585 * t5377 - t4077 - t4080 + t4083 + t5380 + t4089 - t4101 + t4106 + t4108 + t4111 + t5381 - t5382 - t5383;
    (t5375, t5376, t5380, t5381, t5382, t5383, t5384)
}
