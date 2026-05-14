//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 692/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk692<F: Float>(t7365: F, t7398: F, t716: F, t736: F, t7300: F, t7305: F, t7308: F, t7313: F, t7318: F, t7321: F, t7323: F, t7325: F, t7328: F, t7331: F, t7334: F, t7338: F, sigma2: F) -> (F, F, F, F, F) {
    let t7399 = t7365 + t7398;
    let t7400 = t7399 * t716;
    let t7401 = t7400 * sigma2;
    let t7402 = t7401 * t736;
    let t7404 = -t7300 / 24.0 - t7305 / 72.0 - t7308 / 24.0 - t7313 / 576.0 + t7318 / 8.0 + t7321 / 256.0 + t7323 / 24.0 + t7325 / 256.0 - t7328 / 48.0 - t7331 / 9.0 + t7334 / 192.0 - t7338 / 16.0 + t7402 / 16.0;
    (t7399, t7400, t7401, t7402, t7404)
}
