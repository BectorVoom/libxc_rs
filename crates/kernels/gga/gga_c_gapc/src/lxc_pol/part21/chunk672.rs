//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 672/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk672<F: Float>(t517: F, t8356: F, t3954: F, t475: F, t115: F, t8379: F, t4605: F, t8381: F, t3137: F, t6: F, t101: F, t4050: F, t1462: F, t4055: F, t568: F, t8415: F) -> (F, F, F, F, F, F) {
    let t8500 = t8356 * t517;
    let t8501 = t475 * t3954;
    let t8502 = t8500 * t8501;
    let t8504 = t8379 * t115;
    let t8505 = t8504 * t4605;
    let t8506 = t8505 * t8381;
    let t8508 = t6 * t3137;
    let t8509 = t8508 * t101;
    let t8510 = t8509 * t4050;
    let t8511 = t1462 * t4055;
    let t8512 = t8510 * t8511;
    let t8514 = t8415 * t568;
    (t8502, t8506, t8508, t8510, t8512, t8514)
}
