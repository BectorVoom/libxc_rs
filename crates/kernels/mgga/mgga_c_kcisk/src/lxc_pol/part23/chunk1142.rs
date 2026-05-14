//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1142/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1142<F: Float>(t32176: F, t9446: F, t3988: F, t9452: F, t6204: F, t2723: F, t3805: F, t1333: F, t9475: F, t3969: F, t9425: F) -> (F, F, F, F, F, F, F) {
    let t32177 = t9446 * t32176;
    let t32179 = t9452 * t3988;
    let t32180 = t6204 * t32179;
    let t32185 = t3805 * t2723;
    let t32186 = 0.55273148148148148147e-3 * t32185;
    let t32187 = t1333 * t9475;
    let t32189 = t9425 * t3969;
    (t32177, t32179, t32180, t32185, t32186, t32187, t32189)
}
