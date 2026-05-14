//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 803/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk803<F: Float>(t1190: F, t1191: F, t275: F, t9637: F, t9680: F, t2460: F, t4875: F, t2459: F, t4878: F, t4821: F, t1179: F, t2140: F, t4886: F, t2: F, t271: F, t1197: F, t258: F) -> (F, F, F, F, F, F) {
    let t9683 = t1191 * t275 * t9680 + t1190 * t9637;
    let t9689 = 1.28 * t4875 * t2460;
    let t9690 = t2459 * t4878;
    let t9692 = 1.28 * t4821 * t9690;
    let t9695 = t1179 * t2140;
    let t9696 = t9695 * t4886;
    let t9699 = t271 * t2;
    let t9700 = t258 * t1197;
    (t9683, t9689, t9692, t9696, t9699, t9700)
}
