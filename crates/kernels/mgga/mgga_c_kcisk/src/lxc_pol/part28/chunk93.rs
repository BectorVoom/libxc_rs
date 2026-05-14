//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 93/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk93<F: Float>(t2: F, t79: F, t4: F, t20: F, t41: F, t21: F, t122: F, t6: F) -> (F, F, F, F, F) {
    let t341 = t79 * t2;
    let t342 = t341 * t4;
    let t352 = t41 * t20;
    let t353 = t352 * t21;
    let t397 = t122 * t6;
    (t341, t342, t352, t353, t397)
}
