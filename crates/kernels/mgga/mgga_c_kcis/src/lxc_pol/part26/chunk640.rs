//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 640/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk640<F: Float>(t509: F, t7192: F, t552: F, t557: F, t303: F, t2012: F, t5752: F, t1464: F, t3187: F, t3188: F, t6284: F, t8: F) -> (F, F, F, F, F, F, F) {
    let t7193 = t509 * t7192;
    let t7194 = t7193 * t552;
    let t7195 = t7194 * t557;
    let t7196 = t303 * t7195;
    let t7198 = t5752 * t2012;
    let t7199 = t1464 * t7198;
    let t7202 = t6284 * t8 + t3187 + t3188;
    (t7193, t7194, t7195, t7196, t7198, t7199, t7202)
}
