//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 909/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk909<F: Float>(t8785: F, t1105: F, t2160: F, t2158: F, t8799: F, t2148: F, t3729: F, t3725: F, t1108: F, t1112: F, t4529: F, t2151: F, t3734: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11133 = F::new(192.0) * t8785;
    let t11135 = t1105 * t2160;
    let t11136 = F::new(36.0) * t11135;
    let t11139 = t1105 * t2158;
    let t11140 = F::new(36.0) * t11139;
    let t11141 = F::new(96.0) * t8799;
    let t11142 = t2148 * t3729;
    let t11147 = t2148 * t3725;
    let t11149 = t1108 * t2160;
    let t11150 = F::new(96.0) * t11149;
    let t11152 = F::new(96.0) * t1108 * t2158;
    let t11155 = t4529 * t1112;
    let t11156 = F::new(0.0007324578922402618) * t11155;
    let t11157 = t2151 * t3734;
    (t11133, t11136, t11140, t11141, t11142, t11147, t11150, t11152, t11156, t11157)
}
