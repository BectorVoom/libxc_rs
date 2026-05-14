//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 695/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk695<F: Float>(t416: F, t8161: F, t467: F, t471: F, t415: F, t2059: F, t2231: F, t3797: F) -> (F, F, F, F, F) {
    let t8162 = t416 * t8161;
    let t8163 = t8162 * t467;
    let t8164 = t8163 * t471;
    let t8165 = t415 * t8164;
    let t8170 = t2059 * t2231;
    let t8171 = t3797 * t8170;
    (t8162, t8163, t8164, t8165, t8171)
}
