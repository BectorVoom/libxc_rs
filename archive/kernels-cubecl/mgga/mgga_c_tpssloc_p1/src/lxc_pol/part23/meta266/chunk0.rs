//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 936/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk936<F: Float>(t109: F, t20342: F, t656: F, t12747: F, t19471: F, t19480: F, t20305: F, t20308: F, t64: F, t9358: F) -> (F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t20343 = t656 * t20342;
    let t20347 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t9358 - F::cast_from(11.0_f64) / F::cast_from(3.0_f64) * t12747 - F::cast_from(2.0_f64) * t19471 + t19480 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t20305 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t20308 - t64 * t20343 / F::cast_from(8.0_f64));
    (t20343, t20347)
}
