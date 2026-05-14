//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 359/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk359<F: Float>(t1028: F, t1221: F, t914: F, t1034: F, t1059: F, t1099: F, t1101: F, t1106: F, t1186: F, t1188: F, t1210: F, t1216: F, t1220: F, t277: F, t498: F, t95: F) -> (F, F, F) {
    let t1222 = t1221 * t1028;
    let t1223 = t914 * t1222;
    let t1226 = -t1034 + t1059 + t1099 + t1101 - t1106 + 0.25844881434903430496e-2 * t95 * t277 * t1186 * t1188 + t1210 * t498 / 2.0 + t1216 + t1220 * t1223 / 6.0;
    (t1222, t1223, t1226)
}
