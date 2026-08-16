//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 961/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk961<F: Float>(t3237: F, t9189: F, t3234: F, t3151: F, t9044: F, t894: F, t2860: F, t3236: F, t3235: F, t3146: F, t3087: F, t914: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9190 = t9189 * t3237;
    let t9191 = t3234 * t9190;
    let t9193 = t3151 * t9044;
    let t9194 = t894 * t9193;
    let t9197 = t2860 * t3236;
    let t9198 = t3235 * t9197;
    let t9201 = t3146 * t9044;
    let t9202 = t894 * t9201;
    let t9205 = t3087 * t9044;
    let t9206 = t914 * t9205;
    (t9191, t9193, t9194, t9197, t9198, t9201, t9202, t9205, t9206)
}
