//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 834/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk834<F: Float>(t9476: F, t9477: F, t1026: F, t2675: F, t2679: F, t2682: F, t3348: F, t1086: F, t2777: F, t3371: F, t2811: F, t3396: F) -> (F, F, F, F, F, F) {
    let t9478 = t9476 * t9477;
    let t9480 = t2675 * t1026;
    let t9481 = t9480 * t2679;
    let t9483 = t3348 * t2682;
    let t9485 = t1086 * t2777;
    let t9486 = t3371 * t9485;
    let t9488 = t3396 * t2811;
    (t9478, t9481, t9483, t9485, t9486, t9488)
}
