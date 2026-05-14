//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 787/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk787<F: Float>(t139: F, t5911: F, t3516: F, t41: F, t5814: F, t79: F, t5821: F, t163: F, t397: F) -> (F, F, F, F, F) {
    let t11250 = t139 * t5911;
    let t11313 = t139 * t3516 * t41;
    let t11525 = t5814 * t79;
    let t11529 = t5821 * t41;
    let t12261 = t397 * t163;
    (t11250, t11313, t11525, t11529, t12261)
}
