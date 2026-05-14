//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 707/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk707<F: Float>(t2134: F, t27: F, t3118: F, t321: F, t504: F, t7262: F, t507: F, t7191: F, t7229: F, t7364: F, t236: F, t3899: F, t2004: F, t7921: F, t2007: F, t1987: F) -> (F, F, F, F, F, F, F, F) {
    let t36402 = t2134 * t27 * t3118 * t321;
    let t36457 = t504 * t7262;
    let t36471 = t507 * t7191;
    let t36489 = t7229 * t7364;
    let t36504 = t507 * t236 * t3899;
    let t36508 = t7921 * t2004;
    let t36511 = t7921 * t2007;
    let t36513 = t7921 * t1987;
    (t36402, t36457, t36471, t36489, t36504, t36508, t36511, t36513)
}
