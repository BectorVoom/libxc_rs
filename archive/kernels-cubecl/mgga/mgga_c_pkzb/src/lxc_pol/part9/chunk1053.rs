//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1053/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1053<F: Float>(t1769: F, t5312: F, t1726: F, t5389: F, t5393: F, t158: F, t165: F, t5387: F, t1721: F, t5381: F, t5397: F, t1760: F, t5384: F) -> (F, F, F, F, F, F) {
    let t16409 = t1769 * t5312;
    let t16416 = t5389 * t1726;
    let t16417 = t16416 * t5393;
    let t16421 = t158 / t5387 / t165;
    let t16425 = t1721 * t1721;
    let t16438 = t5381 * t5397;
    let t16440 = t5384 * t1760;
    (t16409, t16417, t16421, t16425, t16438, t16440)
}
