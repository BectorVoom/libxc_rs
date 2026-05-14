//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 751/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk751<F: Float>(t1131: F, t3201: F, t2861: F, t3192: F, t3179: F, t251: F, t88: F, t304: F, t86: F, t1123: F, t1085: F, t3225: F, t3221: F, t1094: F, t3168: F, t329: F, t64: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9517 = t3201 * t1131;
    let t9522 = t2861 * t3192;
    let t9524 = t2861 * t3179;
    let t9526 = t88 * t251;
    let t9528 = t86 * t9526 * t304;
    let t9529 = t9528 * t1123;
    let t9531 = t1085 * t3225;
    let t9532 = t9531 * sigma0;
    let t9536 = t2861 * t3221;
    let t9538 = t3168 * t1094;
    let t9539 = t9538 * sigma0;
    let t9543 = t64 * t329;
    (t9517, t9522, t9524, t9526, t9528, t9529, t9531, t9532, t9536, t9538, t9539, t9543)
}
