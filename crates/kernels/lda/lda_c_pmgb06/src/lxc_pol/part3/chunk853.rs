//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 853/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk853<F: Float>(t4556: F, t980: F, t2148: F, t3711: F, t959: F, t3742: F, t968: F, t273: F, t4515: F, t698: F, t1065: F, t2142: F, t248: F, t3890: F, t897: F, t3760: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11160 = t4556 * t980;
    let t11161 = 3.5089341735807875 * t11160;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11165 = 1.7544670867903938 * t11164;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11169 = 51.94757731704439 * t11168;
    let t11171 = t4515 * t273 * t698;
    let t11174 = t248 * t2142 * t1065;
    let t11175 = 3.0 * t11174;
    let t11177 = t248 * t897 * t3890;
    let t11178 = t2148 * t3760;
    (t11161, t11162, t11165, t11166, t11169, t11171, t11175, t11177, t11178)
}
