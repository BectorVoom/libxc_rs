//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 901/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk901<F: Float>(t1056: F, t18574: F, t10096: F, t6272: F, t331: F, t6276: F, t3160: F, t1072: F, t1064: F, t18677: F, t18672: F, t945: F) -> (F, F, F, F, F, F, F) {
    let t19473 = t1056 * t18574;
    let t19476 = t10096 * t6272;
    let t19478 = t331 * t6276;
    let t19480 = t3160 * t6276;
    let t19482 = t1072 * t6272;
    let t19488 = t1064 * t18677;
    let t19491 = t945 * t18672;
    (t19473, t19476, t19478, t19480, t19482, t19488, t19491)
}
