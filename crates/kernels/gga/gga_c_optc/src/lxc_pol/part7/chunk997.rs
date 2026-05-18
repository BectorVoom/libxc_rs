//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 997/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk997<F: Float>(t188: F, t21979: F, t757: F, t103: F, t193: F, t197: F, t2078: F, t102: F, t652: F, t751: F, t133: F, t1928: F) -> (F, F, F, F) {
    let t21981 = t188 * t21979 * t757;
    let t21988 = F::new(261800.0) / F::new(729.0) * t193 * t2078 * t103 * t197;
    let t21989 = t652 * t102;
    let t21991 = t193 * t21989 * t751;
    let t21995 = t193 * t133 * t1928 * t197;
    (t21981, t21988, t21991, t21995)
}
