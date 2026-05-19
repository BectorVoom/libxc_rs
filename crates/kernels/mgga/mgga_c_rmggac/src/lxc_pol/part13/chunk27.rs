//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 27/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk27<F: Float>(t60: F, t62: F, t56: F, t59: F, t49: F, zeta_threshold: F) -> (F, F, F) {
    let t61 = t60 <= zeta_threshold;
    let t63 = t62 * t60;
    let t64 = piecewise3::<F>(t61, t56, t63);
    let t65 = t59 + t64 - F::new(2.0);
    let t68 = F::new(1.0) / t49 / F::new(2.0);
    (t63, t65, t68)
}
