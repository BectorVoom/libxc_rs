//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 503/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk503<F: Float>(t1287: F, t2120: F, t1291: F, t2144: F, t115: F, t2010: F, t155: F, t2156: F, t635: F, t1294: F, t2164: F, t1278: F, t2182: F) -> (F, F, F, F, F, F, F) {
    let t3471 = t2120 * t1287;
    let t3489 = t2144 * t1291;
    let t3491 = t2010 * t115;
    let t3500 = t155 * t2156;
    let t3501 = t3500 * t635;
    let t3504 = t2164 * t1294;
    let t3517 = t2182 * t1278;
    (t3471, t3489, t3491, t3500, t3501, t3504, t3517)
}
