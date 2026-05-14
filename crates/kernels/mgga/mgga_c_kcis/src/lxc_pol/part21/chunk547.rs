//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 547/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk547<F: Float>(t3335: F, t3340: F, t3344: F, t3349: F, t3356: F, t3359: F, t3363: F, t3366: F, t3370: F, t3426: F, t3430: F, t3433: F, t3440: F, t3445: F, t3449: F, t3453: F, t3455: F, t3457: F, t3461: F, t3467: F, t3469: F, t3471: F, t3475: F, t3478: F) -> (F, F) {
    let t3685 = 0.5e0 * t3335 - 0.125e0 * t3340 + 0.625e-1 * t3344 - 0.44965277777777777777e-2 * t3349 - 0.34173611111111111111e0 * t3356 + 0.14388888888888888889e0 * t3359 + 0.91666666666666666667e0 * t3363 - 0.33333333333333333334e0 * t3366 - 0.101171875e-1 * t3370 + 0.9375e-1 * t3426 - 0.20833333333333333333e-1 * t3430 - 0.10791666666666666667e0 * t3433;
    let t3698 = 0.26979166666666666666e-1 * t3440 - 0.20234375e-1 * t3445 - 0.13489583333333333333e-1 * t3449 + 0.101171875e-1 * t3453 + 0.10791666666666666667e0 * t3455 - 0.26979166666666666666e-1 * t3457 - 0.9375e-1 * t3461 + 0.1875e0 * t3467 - 0.5e0 * t3469 + 0.125e0 * t3471 - 0.1875e0 * t3475 + 0.20234375e-1 * t3478;
    (t3685, t3698)
}
