//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 965/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk965<F: Float>(t16046: F, t31: F, t14431: F, t13925: F, t500: F, t8: F, t1697: F, t51: F, t49: F, t75: F, t10: F, t47: F, t204: F, t5401: F, t58: F, t4928: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16047 = t31 * t16046;
    let t16089 = 1.0 / t14431;
    let t16111 = 1.0 / t13925;
    let t16129 = t8 * t500;
    let t16190 = t51 * t1697;
    let t16193 = 0.11483599538271604938e-1 * t49 * t16190 * t75;
    let t16194 = t47 * t10;
    let t16200 = 1.0 / t58 / t16194 * t47 * t5401 * t204 / 48.0;
    let t16202 = t4928 * t500;
    (t16047, t16089, t16111, t16129, t16190, t16193, t16194, t16200, t16202)
}
