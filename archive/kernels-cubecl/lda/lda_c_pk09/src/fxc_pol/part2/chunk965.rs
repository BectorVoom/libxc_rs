//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 965/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk965<F: Float>(t10104: F, t318: F, t1387: F, t9815: F, t1349: F, t309: F, t7766: F, t2143: F, t5794: F, t93: F, t339: F, t9843: F) -> (F, F, F, F, F) {
    let t10223 = t318 * t10104;
    let t10227 = t9815 * t1387;
    let t10240 = t309 * t1349 * t7766;
    let t10243 = t5794 * t2143;
    let t10244 = t93 * t10243;
    let t10249 = t339 * t9843;
    (t10223, t10227, t10240, t10244, t10249)
}
