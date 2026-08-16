//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 339/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk339<F: Float>(t1476: F, t415: F, t1088: F, t1091: F, t1444: F, t1451: F, t1454: F, t1457: F) -> (F, F) {
    let t1477 = t1476 * t415;
    let t1483 = F::cast_from(0.258925e1_f64) * t1451 - t1088 - F::cast_from(0.301925e0_f64) * t1444 + F::cast_from(0.16504875e0_f64) * t1454 - t1091 - F::cast_from(0.82785e-1_f64) * t1457;
    (t1477, t1483)
}
