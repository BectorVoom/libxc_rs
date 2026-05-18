//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 617/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk617<F: Float>(t719: F, t4972: F, t746: F, t741: F, t4803: F, t641: F, t5275: F, t5279: F, t5281: F, t5287: F, t5292: F, t5296: F, t5300: F, t5304: F, t5308: F, t5311: F, t5313: F, t5318: F, t5324: F, t5328: F) -> (F, F, F, F, F, F) {
    let t5330 = F::new(1.0) / t719;
    let t5331 = t5330 * t4972;
    let t5332 = t746 * t5331;
    let t5333 = t741 * t5332;
    let t5335 = t641 * t4803;
    let t5336 = t746 * t5335;
    let t5337 = t741 * t5336;
    let t5339 = t5275 / F::new(16.0) - t5279 / F::new(8.0) + t5281 / F::new(12.0) + t5287 / F::new(8.0) - t5292 / F::new(12.0) - t5296 / F::new(16.0) - t5300 / F::new(72.0) + t5304 / F::new(24.0) - t5308 / F::new(256.0) + t5311 / F::new(128.0) - t5313 / F::new(96.0) - t5318 / F::new(128.0) + t5324 / F::new(96.0) + t5328 / F::new(256.0) - t5333 / F::new(576.0) - t5337 / F::new(192.0);
    (t5330, t5332, t5333, t5336, t5337, t5339)
}
