//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 321/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk321<F: Float>(t1: F, t1382: F, t297: F, t313: F, t1235: F, t897: F, t894: F, t1379: F, t860: F, t862: F, t874: F, t891: F, t893: F) -> (F, F, F, F, F) {
    let t1383 = t1382 * t1;
    let t1384 = t1383 * t297;
    let t1385 = t313 * t1384;
    let t1388 = t897 * t1235;
    let t1389 = t894 * t1388;
    let t1392 = t860 + t862 * t1379 / 288.0 + 0.35500316489081544176e-1 * t874 * t1385 + t891 + 0.18110753103726578864e-2 * t893 * t1389;
    (t1383, t1384, t1388, t1389, t1392)
}
