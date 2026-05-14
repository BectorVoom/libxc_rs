//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 991/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk991<F: Float>(t331: F, t5106: F, t5021: F, t5109: F, t4606: F, t4634: F, t325: F, t4629: F, t2092: F, t933: F, t5084: F, t5087: F, t5097: F, t5100: F, t174: F, t4641: F, t9810: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11762 = t331 * t5106;
    let t11764 = t5021 * t5109;
    let t11770 = t4606 * t4634;
    let t11775 = t325 * t4629;
    let t11781 = t933 * t2092;
    let t11793 = t331 * t5084;
    let t11798 = t331 * t5087;
    let t11803 = t331 * t5097;
    let t11805 = t331 * t5100;
    let t11818 = t174 * t9810 * t4641;
    (t11762, t11764, t11770, t11775, t11781, t11793, t11798, t11803, t11805, t11818)
}
