//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 617/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk617<F: Float>(t719: F, t4972: F, t746: F, t741: F, t4803: F, t641: F, t5275: F, t5279: F, t5281: F, t5287: F, t5292: F, t5296: F, t5300: F, t5304: F, t5308: F, t5311: F, t5313: F, t5318: F, t5324: F, t5328: F) -> (F, F, F, F, F, F) {
    let t5330 = F::cast_from(1.0_f64) / t719;
    let t5331 = t5330 * t4972;
    let t5332 = t746 * t5331;
    let t5333 = t741 * t5332;
    let t5335 = t641 * t4803;
    let t5336 = t746 * t5335;
    let t5337 = t741 * t5336;
    let t5339 = t5275 / F::cast_from(16.0_f64) - t5279 / F::cast_from(8.0_f64) + t5281 / F::cast_from(12.0_f64) + t5287 / F::cast_from(8.0_f64) - t5292 / F::cast_from(12.0_f64) - t5296 / F::cast_from(16.0_f64) - t5300 / F::cast_from(72.0_f64) + t5304 / F::cast_from(24.0_f64) - t5308 / F::cast_from(256.0_f64) + t5311 / F::cast_from(128.0_f64) - t5313 / F::cast_from(96.0_f64) - t5318 / F::cast_from(128.0_f64) + t5324 / F::cast_from(96.0_f64) + t5328 / F::cast_from(256.0_f64) - t5333 / F::cast_from(576.0_f64) - t5337 / F::cast_from(192.0_f64);
    (t5330, t5332, t5333, t5336, t5337, t5339)
}
