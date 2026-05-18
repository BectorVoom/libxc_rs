//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 879/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk879<F: Float>(t7244: F, t8497: F, t7914: F, t8571: F, t1243: F, t236: F, t3351: F, t551: F, t7248: F, t3352: F, t511: F, t5199: F) -> (F, F, F, F) {
    let t39264 = t7244 * t8497;
    let t39265 = F::new(0.19863479950205658386e-4) * t39264;
    let t39266 = t8571 * t7914;
    let t39271 = t3351 * t7248 * t236 * t551 * t1243;
    let t39275 = t3351 * t3352 * t511 * t5199;
    (t39265, t39266, t39271, t39275)
}
