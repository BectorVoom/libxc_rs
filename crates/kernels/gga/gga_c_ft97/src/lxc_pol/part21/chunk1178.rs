//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1178/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1178<F: Float>(t1317: F, t29633: F, t376: F, t1882: F, t29626: F, t29701: F, t432: F, t446: F, t8411: F, t100440: F, t1564: F, t920: F, t23009: F, t29693: F, t29712: F, t5665: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116780 = t1317 * t376 * t29633;
    let t116781 = t116780 / 6.0;
    let t116782 = t1882 * t29626;
    let t116783 = 2.0 / 27.0 * t116782;
    let t116786 = t446 * t8411 * t29701 * t432;
    let t116790 = t446 * t1564 * t100440 * t920;
    let t116793 = t23009 * t376 * t29693;
    let t116794 = t116793 / 8.0;
    let t116796 = t5665 * t376 * t29712;
    (t116780, t116781, t116782, t116783, t116786, t116790, t116793, t116794, t116796)
}
