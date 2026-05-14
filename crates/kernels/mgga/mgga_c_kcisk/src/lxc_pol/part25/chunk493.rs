//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 493/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk493<F: Float>(t227: F, t3293: F, t565: F, t1944: F, sigma2: F, zeta_threshold: F) -> (F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t4573 = piecewise3(t228, 0.0, t3293);
    let t4574 = t565 * t4573;
    let t4581 = t1944 * sigma2;
    (t4573, t4574, t4581)
}
