//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 664/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk664<F: Float>(t2102: F, t9078: F, t143: F, t7760: F, t8277: F, t1985: F, t2: F, t2075: F, t558: F, t582: F, t8266: F, t7368: F) -> (F, F, F, F, F, F) {
    let t9221 = t2102 * t9078;
    let t9224 = t7760 * t143;
    let t9225 = t9224 * t8277;
    let t9230 = t1985 * t2 * t558 * t2075;
    let t9233 = t582 * t8266;
    let t9236 = t7368 * t2;
    (t9221, t9224, t9225, t9230, t9233, t9236)
}
