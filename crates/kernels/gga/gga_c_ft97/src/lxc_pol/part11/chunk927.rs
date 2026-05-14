//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 927/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk927<F: Float>(t2347: F, t41468: F, t2404: F, t92: F, t41473: F, t1771: F, t2410: F, t458: F, t9579: F, t9584: F, t9588: F, t9593: F, t11401: F, t191: F, t26: F, t9573: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41726 = t2347 * t41468;
    let t41728 = t92 * t2404 * t41726;
    let t41731 = t92 * t2404 * t41473;
    let t41733 = t1771 * t2410;
    let t41735 = t458 * t9579;
    let t41737 = t458 * t9584;
    let t41739 = t458 * t9588;
    let t41741 = t458 * t9593;
    let t41743 = t11401 * t191;
    let t41744 = t26 * t41743;
    let t41745 = 280.0 / 81.0 * t41744;
    let t41746 = t458 * t9573;
    (t41726, t41728, t41731, t41733, t41735, t41737, t41739, t41741, t41743, t41744, t41745, t41746)
}
