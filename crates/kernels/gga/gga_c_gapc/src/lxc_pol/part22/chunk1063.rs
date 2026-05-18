//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1063/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1063<F: Float>(t12212: F, t12233: F, t12254: F, t12275: F, t12044: F, t12045: F, t12046: F, t12054: F, t12061: F, t12152: F, t12154: F, t12155: F, t12156: F, t12158: F, t12161: F, t12162: F, t12192: F, t2464: F, t3846: F, t884: F) -> (F, F) {
    let t12277 = t12212 + t12233 + t12254 + t12275;
    let t12279 = -t12277 * t884 - t2464 * t3846 - t12044 + t12045 + t12046 + t12054 + t12061 - t12152 - t12154 - t12155 - t12156 + t12158 + t12161 - t12162 + t12192;
    (t12277, t12279)
}
