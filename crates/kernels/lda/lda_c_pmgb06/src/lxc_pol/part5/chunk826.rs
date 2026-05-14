//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 826/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk826<F: Float>(t1108: F, t2160: F, t2158: F, t1112: F, t4529: F, t2151: F, t3734: F, t4556: F, t980: F, t2148: F, t3711: F, t959: F, t3742: F, t968: F, t1065: F, t2142: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11149 = t1108 * t2160;
    let t11150 = 96.0 * t11149;
    let t11152 = 96.0 * t1108 * t2158;
    let t11155 = t4529 * t1112;
    let t11156 = 0.0007324578922402618 * t11155;
    let t11157 = t2151 * t3734;
    let t11160 = t4556 * t980;
    let t11161 = 3.5089341735807875 * t11160;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11165 = 1.7544670867903938 * t11164;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11169 = 51.94757731704439 * t11168;
    let t11174 = t248 * t2142 * t1065;
    (t11150, t11152, t11156, t11157, t11161, t11162, t11165, t11166, t11169, t11174)
}
