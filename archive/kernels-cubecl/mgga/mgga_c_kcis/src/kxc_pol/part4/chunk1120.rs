//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1120/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1120<F: Float>(t3255: F, t4634: F, t330: F, t4670: F, t829: F, t3274: F, t2635: F, t4632: F, t1727: F, t2844: F, t2630: F, t10297: F) -> (F, F, F, F) {
    let t14137 = t3255 * t4634;
    let t14139 = t4670 * t330;
    let t14140 = t14139 * t829;
    let t14141 = t3274 * t14140;
    let t14144 = t4632 * t2635;
    let t14145 = t3274 * t14144;
    let t14148 = t1727 * t2844;
    let t14149 = t14148 * t2630;
    let t14150 = t10297 * t14149;
    (t14137, t14141, t14145, t14150)
}
