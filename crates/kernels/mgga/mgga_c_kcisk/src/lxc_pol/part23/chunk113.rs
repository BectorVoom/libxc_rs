//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 113/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk113<F: Float>(t25: F, t313: F, t353: F, t344: F, t347: F, t350: F) -> (F, F, F, F) {
    let t355 = t353 * t25 * t313;
    let t357 = 0.379785e1 * t347 + 0.8969e0 * t344 + 0.204775e0 * t350 + 0.24647e0 * t355;
    let t360 = 1.0 + 0.16081824322151104822e2 / t357;
    let t361 = f64::ln(t360);
    (t355, t357, t360, t361)
}
