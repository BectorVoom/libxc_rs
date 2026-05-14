//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 633/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk633<F: Float>(t5991: F, t6332: F, t6331: F, t1512: F, t2274: F, t1504: F, t499: F, t5967: F, t498: F, t467: F, t5866: F, t492: F, t500: F, t1513: F, t2271: F, t1497: F, t2259: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6333 = t6332 * t5991;
    let t6334 = t6331 * t6333;
    let t6336 = t1512 * t2274;
    let t6337 = t1504 * t6336;
    let t6339 = t499 * t5967;
    let t6340 = t498 * t6339;
    let t6341 = t1504 * t6340;
    let t6343 = t5866 * t467;
    let t6344 = t6343 * t492;
    let t6345 = t6344 * t500;
    let t6347 = t2271 * t1513;
    let t6349 = t2259 * t1497;
    (t6333, t6334, t6336, t6337, t6340, t6341, t6343, t6344, t6345, t6347, t6349)
}
