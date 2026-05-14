//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1149/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1149<F: Float>(t28150: F, t8392: F, t1882: F, t28120: F, t28257: F, t28319: F, t28392: F, t28195: F, t28230: F, t28243: F, t28430: F, t28171: F, t28102: F, t28379: F, t1449: F, t9577: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t111225 = 2.0 / 27.0 * t8392 * t28150;
    let t111227 = 2.0 / 9.0 * t1882 * t28120;
    let t111237 = 2.0 / 9.0 * t1882 * t28257;
    let t111239 = 2.0 / 9.0 * t1882 * t28319;
    let t111241 = 2.0 / 9.0 * t1882 * t28392;
    let t111252 = 2.0 / 9.0 * t1882 * t28195;
    let t111254 = 2.0 / 9.0 * t1882 * t28230;
    let t111256 = 2.0 / 9.0 * t1882 * t28243;
    let t111262 = 4.0 / 9.0 * t1882 * t28430;
    let t111264 = 2.0 / 9.0 * t1882 * t28171;
    let t111266 = 2.0 / 9.0 * t1882 * t28102;
    let t111276 = 2.0 / 27.0 * t8392 * t28379;
    let t111283 = t1449 * t9577;
    (t111225, t111227, t111237, t111239, t111241, t111252, t111254, t111256, t111262, t111264, t111266, t111276, t111283)
}
