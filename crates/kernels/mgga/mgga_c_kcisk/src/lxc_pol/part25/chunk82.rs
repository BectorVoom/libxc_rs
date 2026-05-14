//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 82/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk82<F: Float>(t227: F, t229: F, t37: F, t226: F, t44: F, t41: F, zeta_threshold: F) -> (F, F) {
    let t228 = t227 <= zeta_threshold;
    let t231 = piecewise3(t228, t37, t229 * t227);
    let t233 = (t226 + t231 - 2.0) * t44;
    let t236 = piecewise3(2.0 <= zeta_threshold, t37, 2.0 * t41);
    let t238 = piecewise3(0.0 <= zeta_threshold, t37, 0.0);
    let t240 = (t236 + t238 - 2.0) * t44;
    (t233, t240)
}
