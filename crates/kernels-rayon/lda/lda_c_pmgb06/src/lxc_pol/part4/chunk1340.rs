//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1340/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1340(t17457: f64, t5068: f64, t5139: f64, t13068: f64, t5138: f64, t1531: f64, t2106: f64, t5077: f64, t5086: f64, t15862: f64, t5091: f64, t5118: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t17607 = 4.0_f64 / 15.0_f64 * t5068 * t5139 * t17457;
    let t17610 = 4.0_f64 / 9.0_f64 * t5138 * t13068 * t17457;
    let t17614 = 8.0_f64 / 45.0_f64 * t5077 * t2106 * t1531 * t5086;
    let t17616 = 8.0_f64 / 45.0_f64 * t15862 * t5091;
    let t17617 = t5118 * t822;
    (t17607, t17610, t17614, t17616, t17617)
}
