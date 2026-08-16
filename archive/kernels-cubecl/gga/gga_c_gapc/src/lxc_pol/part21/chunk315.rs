//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 315/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk315<F: Float>(t1302: F, t1303: F, t106: F, t78: F, t14: F, t60: F, t159: F, t88: F, t108: F, t348: F, t1147: F, t391: F) -> (F, F, F, F, F, F) {
    let t1304 = t1302 * t1303;
    let t1308 = t78 * t106;
    let t1312 = t60 * t14;
    let t1319 = t159 * t88;
    let t1320 = t348 * t108;
    let t1326 = t391 * t1147;
    (t1304, t1308, t1312, t1319, t1320, t1326)
}
