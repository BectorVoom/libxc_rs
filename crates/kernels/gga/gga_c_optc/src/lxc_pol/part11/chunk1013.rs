//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1013/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1013<F: Float>(t1179: F, t45693: F, t5297: F, t7878: F, t1162: F, t5318: F, t7274: F, t140: F, t2665: F, t5255: F, t3183: F, t3101: F, t12489: F, t4444: F, t12726: F, t4450: F) -> (F, F, F, F, F, F, F, F) {
    let t45694 = t1179 * t45693;
    let t45718 = t7878 * t5297;
    let t45719 = t1179 * t45718;
    let t45731 = t1162 * t7274 * t5318;
    let t45769 = t5255 * t2665 * t140;
    let t45770 = t3183 * t45769;
    let t45773 = t3101 * t45769;
    let t45788 = t4444 * t12489;
    let t45795 = t4450 * t12726;
    (t45694, t45718, t45719, t45731, t45770, t45773, t45788, t45795)
}
