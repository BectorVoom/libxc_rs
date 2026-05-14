//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 856/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk856<F: Float>(t10314: F, t10330: F, t323: F, t306: F, t1215: F, t2551: F, t130: F, t2550: F, t93: F, t1593: F, t2487: F, t10104: F, t327: F, t1435: F, t2583: F, t5047: F, t5071: F, t5989: F, t6002: F, t6003: F, t6008: F, t6011: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F, F, F, F, F, F, F) {
    let t10331 = t10314 + t10330;
    let t10332 = t323 * t10331;
    let t10333 = t10332 * t306;
    let t10341 = t2551 * t1215;
    let t10345 = t130 * t2550;
    let t10346 = t93 * t10345;
    let t10349 = t1593 * t2487;
    let t10352 = t327 * t10104;
    let t10355 = t2583 * t1435;
    let t10369 = -t6003 + t6008 + t5989 + t6002 + 0.2946275542389858 * t5047 - t6011 + 0.0982091847463286 * t5071 + 2.9540870317630623 * t9922 - 2.9540870317630623 * t9925 - 2.9540870317630623 * t9929 + 4.431130547644593 * t9933 - 2.9540870317630623 * t9936 + 0.2946275542389858 * t9746 + 0.0982091847463286 * t9753 + 0.2946275542389858 * t9756 + 0.5892551084779716 * t9628 - 0.9846956772543541 * t9943;
    (t10333, t10341, t10346, t10349, t10352, t10355, t10369)
}
