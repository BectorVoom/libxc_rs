//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 153/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk153<F: Float>(t227: F, tau1: F, zeta_threshold: F) -> (F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t565 = F::new(1.0) / tau1;
    let t566 = piecewise3::<F>(t228, zeta_threshold, t227);
    let t567 = t565 * t566;
    let t568 = F::new(1.0) / t227;
    (t565, t567, t568)
}
