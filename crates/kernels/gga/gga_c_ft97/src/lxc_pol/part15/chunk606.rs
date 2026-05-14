//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 606/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk606<F: Float>(t1775: F, t4768: F, t4759: F, t458: F, t4776: F, t4772: F, t4762: F, t2112: F, t358: F, t16925: F, t16928: F, t1882: F, t4819: F, t4815: F, t4790: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t17274 = t1775 * t4768;
    let t17276 = t1775 * t4759;
    let t17279 = t458 * t4776;
    let t17281 = t458 * t4772;
    let t17310 = t1775 * t4762;
    let t17338 = t2112 * t358;
    let t17351 = t16925 / 3.0;
    let t17352 = 2.0 / 3.0 * t16928;
    let t17360 = t1882 * t4819;
    let t17362 = t1882 * t4815;
    let t17409 = t4790 * t604;
    (t17274, t17276, t17279, t17281, t17310, t17338, t17351, t17352, t17360, t17362, t17409)
}
