//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 62/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk62<F: Float>(t53: F, t60: F, t155: F, t184: F, t156: F, t181: F, t55: F, t57: F, t62: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t185 = t155 * t184;
    let t187 = F::new(0.19751673498613801407e-1) * t156 * t181;
    let t188 = t55 * t55;
    let t189 = t57 * t57;
    let t190 = piecewise3::<f64>(t54, t188, t189);
    let t191 = t62 * t62;
    let t192 = piecewise3::<f64>(t61, t188, t191);
    let t194 = t190 / F::new(2.0) + t192 / F::new(2.0);
    (t185, t187, t189, t191, t194)
}
