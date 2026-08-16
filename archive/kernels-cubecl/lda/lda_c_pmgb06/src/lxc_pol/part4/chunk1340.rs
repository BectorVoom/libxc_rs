//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1340/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1340<F: Float>(t17457: F, t5068: F, t5139: F, t13068: F, t5138: F, t1531: F, t2106: F, t5077: F, t5086: F, t15862: F, t5091: F, t5118: F, t822: F) -> (F, F, F, F, F) {
    let t17607 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t5068 * t5139 * t17457;
    let t17610 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5138 * t13068 * t17457;
    let t17614 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5077 * t2106 * t1531 * t5086;
    let t17616 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t15862 * t5091;
    let t17617 = t5118 * t822;
    (t17607, t17610, t17614, t17616, t17617)
}
