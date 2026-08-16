//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1110/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1110<F: Float>(t115: F, t2770: F, t5274: F, t3209: F, t1724: F, t12597: F, t4501: F, t5336: F, t7878: F, t1179: F, t5297: F, t1162: F, t5318: F, t7274: F) -> (F, F, F, F, F, F, F, F) {
    let t45438 = t5274 * t2770 * t115;
    let t45439 = t3209 * t45438;
    let t45442 = t1724 * t45438;
    let t45584 = t4501 * t12597;
    let t45693 = t7878 * t5336;
    let t45694 = t1179 * t45693;
    let t45718 = t7878 * t5297;
    let t45719 = t1179 * t45718;
    let t45731 = t1162 * t7274 * t5318;
    (t45439, t45442, t45584, t45693, t45694, t45718, t45719, t45731)
}
