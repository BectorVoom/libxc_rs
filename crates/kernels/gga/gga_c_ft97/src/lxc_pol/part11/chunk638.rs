//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 638/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk638<F: Float>(t9152: F, t9309: F, t9400: F, t9458: F, t160: F, t9394: F, t149: F, t165: F, t1953: F, t2081: F, t2228: F, t564: F, t614: F, t8788: F, t8790: F, t9084: F, t9149: F, t9259: F, t9277: F, t9289: F, t9429: F, t9441: F) -> (F, F, F) {
    let t9460 = t9152 + t9309 + t9400 + t9458;
    let t9462 = t9394 * t160;
    let t9470 = -t149 * t9460 - t165 * t8788 - 2.0 * t165 * t8790 - t165 * t9084 - 3.0 * t1953 * t614 - 3.0 * t2081 * t614 - 3.0 * t2228 * t564 - 6.0 * t9149 - 2.0 * t9259 + 12.0 * t9277 + 12.0 * t9289 - 6.0 * t9429 - 12.0 * t9441 + 2.0 * t9462;
    (t9460, t9462, t9470)
}
