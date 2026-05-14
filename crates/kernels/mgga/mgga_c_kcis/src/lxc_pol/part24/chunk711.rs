//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 711/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk711<F: Float>(t1130: F, t2844: F, t1131: F, t3201: F, t251: F, t88: F, t304: F, t86: F, t1123: F, t1085: F, t3225: F, t329: F, t64: F, t358: F, t283: F, t1135: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9512 = t1130 * t2844;
    let t9517 = t3201 * t1131;
    let t9526 = t88 * t251;
    let t9528 = t86 * t9526 * t304;
    let t9529 = t9528 * t1123;
    let t9531 = t1085 * t3225;
    let t9532 = t9531 * sigma0;
    let t9543 = t64 * t329;
    let t9545 = 1.0 / t358 / t9543;
    let t9546 = t283 * t9545;
    let t9552 = t9528 * t1135;
    (t9512, t9517, t9526, t9528, t9529, t9531, t9532, t9545, t9546, t9552)
}
