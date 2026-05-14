//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 950/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk950<F: Float>(t2083: F, t3532: F, t13148: F, t3278: F, t16940: F, t5915: F, t3559: F, t5948: F, t5926: F, t3521: F, t5929: F, t13138: F, t2075: F, t3593: F, t19127: F, t5907: F) -> (F, F, F, F, F, F, F) {
    let t19266 = t2083 * t3532;
    let t19268 = t13148 * t19266 * t3278;
    let t19271 = t16940 * t5915;
    let t19273 = t5948 * t3559;
    let t19274 = t5926 * t19273;
    let t19278 = 0.98556445e-3 * t3521 * t5929;
    let t19280 = t13138 * t2075 * t3593;
    let t19283 = t5907 * t19127;
    (t19268, t19271, t19273, t19274, t19278, t19280, t19283)
}
