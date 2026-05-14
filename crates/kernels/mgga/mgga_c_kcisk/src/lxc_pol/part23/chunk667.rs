//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 667/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk667<F: Float>(t416: F, t5866: F, t467: F, t471: F, t415: F, t2173: F, t3924: F) -> (F, F, F, F, F) {
    let t5867 = t416 * t5866;
    let t5868 = t5867 * t467;
    let t5869 = t5868 * t471;
    let t5870 = t415 * t5869;
    let t5874 = t2173 * t3924;
    (t5867, t5868, t5869, t5870, t5874)
}
