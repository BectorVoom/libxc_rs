//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 746/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk746<F: Float>(t1797: F, t9529: F, t1256: F, t2226: F, t108: F, t176: F, t203: F, t131: F, t2020: F, t160: F, t658: F, t1271: F, t141: F) -> (F, F, F, F, F, F) {
    let t9530 = t9529 * t1797;
    let t9532 = t2226 * t1256;
    let t9534 = t176 * t9532 * t108;
    let t9535 = t9534 * t203;
    let t9598 = t2020 * t131;
    let t9599 = t160 * t658;
    let t9600 = t9598 * t9599;
    let t9601 = t141 * t1271;
    (t9530, t9534, t9535, t9599, t9600, t9601)
}
