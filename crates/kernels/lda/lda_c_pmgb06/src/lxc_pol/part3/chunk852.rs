//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 852/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk852<F: Float>(t1105: F, t2158: F, t8799: F, t2148: F, t3729: F, t27: F, t4515: F, t693: F, t3725: F, t1108: F, t2160: F, t1112: F, t4529: F, t2151: F, t3734: F, t8798: F, t8814: F, t8822: F, t8824: F, t8826: F, t8830: F, t8834: F) -> (F,) {
    let t11139 = t1105 * t2158;
    let t11140 = 36.0 * t11139;
    let t11141 = 96.0 * t8799;
    let t11142 = t2148 * t3729;
    let t11145 = t4515 * t27 * t693;
    let t11147 = t2148 * t3725;
    let t11149 = t1108 * t2160;
    let t11150 = 96.0 * t11149;
    let t11152 = 96.0 * t1108 * t2158;
    let t11155 = t4529 * t1112;
    let t11156 = 0.0007324578922402618 * t11155;
    let t11157 = t2151 * t3734;
    let t11159 = t11140 - t8798 - t11141 + 3.5089341735807875 * t11142 - 0.0005493434191801964 * t11145 - 51.94757731704439 * t11147 - t11150 - t11152 + t8814 + t8822 - 1.7544670867903938 * t8824 - 51.94757731704439 * t8826 + t8830 - t8834 + t11156 - 0.0005696894717424259 * t11157;
    (t11159,)
}
