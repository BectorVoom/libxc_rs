//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1034/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1034<F: Float>(t13265: F, t3338: F, t5046: F, t10787: F, t5091: F, t14590: F, t3337: F, t14092: F, t5047: F, t1133: F, t4984: F, t5181: F, t3437: F, t1797: F, t3362: F, t1176: F, t5165: F) -> (F, F, F, F, F, F, F, F) {
    let t14695 = t3338 * t13265;
    let t14696 = t5046 * t14695;
    let t14698 = t10787 * t5091;
    let t14700 = t3338 * t14590;
    let t14701 = t3337 * t14700;
    let t14703 = t5047 * t14092;
    let t14704 = t5046 * t14703;
    let t14706 = t4984 * t1133;
    let t14707 = t5181 * t14706;
    let t14708 = t3437 * t14707;
    let t14710 = t1797 * t3362;
    let t14712 = t5165 * t1176;
    (t14696, t14698, t14701, t14704, t14706, t14708, t14710, t14712)
}
