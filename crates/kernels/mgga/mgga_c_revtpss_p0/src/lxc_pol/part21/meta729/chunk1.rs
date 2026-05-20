//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2573/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2573<F: Float>(t2661: F, t5675: F, t9929: F, t9934: F, t9775: F, t9981: F, t1398: F, t3992: F, t4010: F, t9956: F, t3938: F, t47218: F) -> (F, F, F, F) {
    let t47318 = t2661 * t9934 * t9929 * t5675;
    let t47320 = t9775 * t9981;
    let t47325 = t2661 * t3992 * t4010 * t1398 * t9956;
    let t47329 = t2661 * t3992 * t47218 * t3938;
    (t47318, t47320, t47325, t47329)
}
