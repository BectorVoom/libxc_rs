//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1243/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1243<F: Float>(t32459: F, t6480: F, t32458: F, t1596: F, t2059: F, t32465: F, t32464: F) -> (F, F, F, F) {
    let t33905 = t32459 * t6480;
    let t33906 = t32458 * t33905;
    let t33909 = t2059 * t1596;
    let t33910 = t32465 * t33909;
    let t33911 = t32464 * t33910;
    (t33905, t33906, t33910, t33911)
}
