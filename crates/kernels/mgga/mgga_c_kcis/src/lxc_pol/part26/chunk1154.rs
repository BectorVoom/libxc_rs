//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1154/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1154<F: Float>(t29400: F, t541: F, t303: F, t1364: F, t23157: F, t5742: F, t6140: F, t2237: F, t2239: F, t27483: F, t27486: F, t28471: F, t28508: F, t28522: F, t28547: F, t29300: F, t29381: F, t29384: F, t29387: F, t29393: F, t29397: F) -> (F, F, F, F, F) {
    let t29401 = t541 * t29400;
    let t29402 = t303 * t29401;
    let t29404 = t1364 * t23157;
    let t29407 = t5742 * t6140;
    let t29410 = F::new(0.12356481481481481482e-2) * t28471 - F::new(0.88437037037037037034e-2) * t28508 - F::new(0.24872916666666666666e-2) * t29381 + F::new(0.49745833333333333332e-2) * t29384 + F::new(0.13265555555555555555e-1) * t29387 + F::new(0.69505208333333333333e-3) * t2237 * t29300 - t27483 + t27486 + F::new(0.46336805555555555556e-3) * t28522 - F::new(0.33163888888888888888e-2) * t28547 - F::new(0.69505208333333333333e-3) * t29393 * t2239 - F::new(0.13265555555555555555e-1) * t29397 + F::new(0.24320185185185185185e-1) * t29402 - F::new(0.67960648148148148147e-2) * t29404 * t2239 + F::new(0.37069444444444444444e-2) * t29407 * t2239;
    (t29401, t29402, t29404, t29407, t29410)
}
