//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 108/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk108<F: Float>(t344: F, t20: F, t41: F, t21: F, t25: F, t313: F, t347: F) -> (F, F, F, F, F, F, F) {
    let t350 = pow_3_2(t344);
    let t352 = t41 * t20;
    let t353 = t352 * t21;
    let t355 = t353 * t25 * t313;
    let t357 = 0.379785e1 * t347 + 0.8969e0 * t344 + 0.204775e0 * t350 + 0.24647e0 * t355;
    let t360 = 1.0 + 0.16081824322151104822e2 / t357;
    let t361 = f64::ln(t360);
    (t350, t352, t353, t355, t357, t360, t361)
}
