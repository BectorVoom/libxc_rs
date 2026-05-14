//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1167/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1167<F: Float>(t1882: F, t28195: F, t28230: F, t28243: F, t28430: F, t28171: F, t28102: F, t28379: F, t8392: F, t1449: F, t9577: F, t108366: F, t108439: F, t13863: F, t13885: F, t13927: F, t14163: F, t14187: F, t14192: F, t14213: F, t14259: F, t1901: F, t24594: F, t24717: F, t24737: F, t2574: F, t28140: F, t3977: F, t446: F, t51669: F, t53797: F, t6074: F, t729: F, t97705: F) -> (F,) {
    let t111252 = 2.0 / 9.0 * t1882 * t28195;
    let t111254 = 2.0 / 9.0 * t1882 * t28230;
    let t111256 = 2.0 / 9.0 * t1882 * t28243;
    let t111262 = 4.0 / 9.0 * t1882 * t28430;
    let t111264 = 2.0 / 9.0 * t1882 * t28171;
    let t111266 = 2.0 / 9.0 * t1882 * t28102;
    let t111276 = 2.0 / 27.0 * t8392 * t28379;
    let t111283 = t1449 * t9577;
    let t111288 = -2.0 / 3.0 * t446 * t2574 * t3977 * t24717 + 4.0 / 9.0 * t53797 * t97705 * t14192 + t111252 + t111254 - t111256 - 2.0 / 3.0 * t446 * t729 * t13927 * t24594 - t111262 + t111264 + t111266 - 2.0 / 3.0 * t1901 * t13885 * t24737 * t14259 - 2.0 * t1901 * t28140 * t6074 * t14213 + t111276 - 2.0 / 9.0 * t1901 * t14163 * t108439 - 4.0 / 27.0 * t1901 * t51669 * t108366 - 4.0 / 9.0 * t1901 * t14187 * t111283 * t13863;
    (t111288,)
}
