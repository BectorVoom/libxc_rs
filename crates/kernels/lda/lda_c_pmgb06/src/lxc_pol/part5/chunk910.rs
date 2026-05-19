//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 910/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk910<F: Float>(t4556: F, t980: F, t2148: F, t3711: F, t959: F, t3742: F, t968: F, t1065: F, t2142: F, t248: F, t3890: F, t897: F) -> (F, F, F, F, F, F, F) {
    let t11160 = t4556 * t980;
    let t11161 = F::cast_from(3.5089341735807875_f64) * t11160;
    let t11162 = t2148 * t3711;
    let t11164 = t4556 * t959;
    let t11165 = F::cast_from(1.7544670867903938_f64) * t11164;
    let t11166 = t2148 * t3742;
    let t11168 = t4556 * t968;
    let t11169 = F::cast_from(51.94757731704439_f64) * t11168;
    let t11174 = t248 * t2142 * t1065;
    let t11175 = F::new(3.0) * t11174;
    let t11177 = t248 * t897 * t3890;
    (t11161, t11162, t11165, t11166, t11169, t11175, t11177)
}
