//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1105/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1105<F: Float>(t109: F, t209: F, t2196: F, t2193: F, t10819: F, t421: F) -> (F, F, F) {
    let t26971 = t209 * t109;
    let t26972 = t26971 * t2196;
    let t26974 = F::new(0.7722800925925925926e-4) * t2193 * t26972;
    let t26975 = t421 * t10819;
    (t26972, t26974, t26975)
}
