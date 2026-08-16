//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 135/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk135<F: Float>(t53: F, t280: F, t437: F, t62: F, zeta_threshold: F) -> (F, F) {
    let t54 = t53 <= zeta_threshold;
    let t440 = piecewise3::<F>(t54, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t437 * t280);
    let t441 = F::cast_from(1.0_f64) / t62;
    (t440, t441)
}
