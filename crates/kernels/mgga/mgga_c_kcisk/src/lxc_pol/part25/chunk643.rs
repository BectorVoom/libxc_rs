//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 643/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk643<F: Float>(t4637: F, t4638: F, t6756: F, t6761: F, t6766: F, t6769: F) -> (F,) {
    let t6771 = t4637 + t4638 / 9.0 + t6756 / 9.0 - 2.0 / 9.0 * t6761 + 2.0 / 3.0 * t6766 + 2.0 / 3.0 * t6769;
    (t6771,)
}
