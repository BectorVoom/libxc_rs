//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 30/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk30<F: Float>(t50: F, t52: F, t46: F, t49: F, zeta_threshold: F) -> (F, F, F) {
    let cbrt2 = F::cast_from(M_CBRT2);
    let t51 = t50 <= zeta_threshold;
    let t53 = t52 * t50;
    let t54 = piecewise3::<F>(t51, t46, t53);
    let t55 = t49 + t54 - F::new(2.0);
    let t56 = cbrt2;
    (t53, t55, t56)
}
