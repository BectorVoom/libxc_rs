//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1001/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1001<F: Float>(t110: F, t1756: F, t1759: F, t209: F, t6475: F, t6481: F, t115: F, t6568: F, t103: F, t193: F, t197: F, t2078: F) -> (F, F, F, F) {
    let t21903 = F::cast_from(0.28493333333333333334e0_f64) * t209 * t110 * t1756 * t1759;
    let t21907 = F::cast_from(0.4274e0_f64) * t209 * t6481 * t6475;
    let t21979 = t6568 * t115;
    let t21988 = F::cast_from(261800.0_f64) / F::cast_from(729.0_f64) * t193 * t2078 * t103 * t197;
    (t21903, t21907, t21979, t21988)
}
