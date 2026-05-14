//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 976/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk976<F: Float>(t16190: F, t49: F, t75: F, t10: F, t47: F, t204: F, t5401: F, t58: F, t4928: F, t500: F, t1476: F, t170: F, t1475: F, t1697: F, t475: F, t474: F) -> (F, F, F, F, F, F, F, F) {
    let t16193 = 0.11483599538271604938e-1 * t49 * t16190 * t75;
    let t16194 = t47 * t10;
    let t16200 = 1.0 / t58 / t16194 * t47 * t5401 * t204 / 48.0;
    let t16202 = t4928 * t500;
    let t16204 = t1476 * t170;
    let t16205 = t1475 * t16204;
    let t16207 = t475 * t1697;
    let t16208 = t474 * t16207;
    (t16193, t16194, t16200, t16202, t16204, t16205, t16207, t16208)
}
