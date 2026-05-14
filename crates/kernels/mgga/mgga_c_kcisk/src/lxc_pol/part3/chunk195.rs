//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 195/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk195<F: Float>(t746: F, t747: F, t741: F, t737: F, t724: F, t571: F) -> (F, F, F, F, F, F) {
    let t748 = t746 * t747;
    let t749 = t741 * t748;
    let t751 = 1.0 + t737 / 16.0 - t749 / 256.0;
    let t752 = 1.0 / t751;
    let t753 = t724 * t752;
    let t755 = 1.0 + 0.5137e-1 * t571;
    (t748, t749, t751, t752, t753, t755)
}
