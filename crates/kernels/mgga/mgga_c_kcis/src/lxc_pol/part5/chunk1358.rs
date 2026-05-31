//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1358/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1358<F: Float>(t22373: F, t6027: F, t17382: F, t21910: F, t5908: F, t22349: F, t22352: F, t22355: F, t22359: F, t22362: F, t22365: F, t22367: F, t22369: F, t22371: F) -> (F, F, F) {
    let t22374 = t6027 * t22373;
    let t22376 = t17382 * t21910;
    let t22377 = t5908 * t22376;
    let t22379 = -t22349 / F::cast_from(128.0_f64) + t22352 / F::cast_from(4.0_f64) + t22355 / F::cast_from(288.0_f64) - t22359 / F::cast_from(16.0_f64) + t22362 / F::cast_from(8.0_f64) + t22365 / F::cast_from(192.0_f64) - t22367 / F::cast_from(18.0_f64) - t22369 / F::cast_from(8.0_f64) - t22371 / F::cast_from(18.0_f64) + t22374 / F::cast_from(12.0_f64) + t22377 / F::cast_from(54.0_f64);
    (t22374, t22377, t22379)
}
