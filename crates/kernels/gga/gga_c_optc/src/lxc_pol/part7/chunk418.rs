//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 418/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk418<F: Float>(t131: F, t2078: F, t130: F, t142: F, t654: F, t661: F, t103: F, t137: F) -> (F, F, F, F, F) {
    let t2079 = t131 * t2078;
    let t2080 = t130 * t2079;
    let t2082 = 0.71839320644782096162e-1 * t2080 * t142;
    let t2083 = t654 * t661;
    let t2085 = t137 * t103;
    let t2086 = 1.0 / t2085;
    (t2079, t2080, t2082, t2083, t2086)
}
