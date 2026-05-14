//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 668/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk668<F: Float>(t2010: F, t623: F, t56: F, t658: F, t111: F, t2003: F, t627: F, t668: F, t145: F, t128: F, t2155: F, t131: F, t133: F, t155: F, t108: F, t2156: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6941 = t623 * t2010;
    let t6944 = t56 * t658;
    let t6945 = t111 * t6944;
    let t6956 = t2003 * t627;
    let t6975 = t668 * t668;
    let t6976 = 1.0 / t6975;
    let t6977 = t145 * t6976;
    let t6990 = 1.0 / t2155 / t128;
    let t6991 = t6990 * t131;
    let t6993 = t155 * t6991 * t133;
    let t7000 = t2156 * t108;
    (t6941, t6944, t6945, t6956, t6975, t6976, t6977, t6990, t6991, t6993, t7000)
}
