//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 570/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk570<F: Float>(t5: F, t1437: F, t2240: F, t3953: F, t5385: F, t5389: F, t5445: F, t605: F, t86: F, t112: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t5449 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) * t1437 * t3953 + F::cast_from(20.0_f64) * t2240 * t5389 + t5385 * t86 - F::cast_from(4.0_f64) * t5445 * t605);
    let t5450 = t5449 * t112;
    (t5449, t5450)
}
