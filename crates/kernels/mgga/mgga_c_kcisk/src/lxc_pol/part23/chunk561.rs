//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 561/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk561<F: Float>(t3906: F, t416: F, t467: F, t471: F, t415: F, t392: F, t494: F) -> (F, F, F, F, F) {
    let t3907 = t416 * t3906;
    let t3908 = t3907 * t467;
    let t3909 = t3908 * t471;
    let t3910 = t415 * t3909;
    let t3913 = 1.0 / t392 / t494;
    (t3907, t3908, t3909, t3910, t3913)
}
