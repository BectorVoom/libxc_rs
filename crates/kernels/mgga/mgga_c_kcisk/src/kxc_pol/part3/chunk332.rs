//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 332/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk332<F: Float>(t1634: F, t1657: F, t1638: F, t1649: F, t1654: F, t1661: F) -> (F,) {
    let t1678 = 0.301925e0 * t1634;
    let t1681 = 0.82785e-1 * t1657;
    let t1683 = 0.258925e1 * t1649 - t1678 - 0.301925e0 * t1638 + 0.16504875e0 * t1654 - t1681 - 0.82785e-1 * t1661;
    (t1683,)
}
