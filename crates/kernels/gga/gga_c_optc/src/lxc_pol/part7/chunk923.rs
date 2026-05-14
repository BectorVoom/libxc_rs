//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 923/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk923<F: Float>(t6681: F, t732: F, t737: F, t188: F, t1955: F, t6680: F, t1917: F, t2229: F, t2234: F, t115: F, t6568: F, t757: F, t103: F, t193: F, t197: F, t2078: F) -> (F, F, F, F, F, F, F) {
    let t21968 = t732 * t6681;
    let t21970 = t737 * t6681;
    let t21973 = t188 * t6680 * t1955;
    let t21975 = t2229 * t1917;
    let t21977 = t2234 * t1917;
    let t21979 = t6568 * t115;
    let t21981 = t188 * t21979 * t757;
    let t21988 = 261800.0 / 729.0 * t193 * t2078 * t103 * t197;
    (t21968, t21970, t21973, t21975, t21977, t21981, t21988)
}
