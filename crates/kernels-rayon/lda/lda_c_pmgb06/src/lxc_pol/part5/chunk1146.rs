//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1146/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1146(t132: f64, t137: f64, t2106: f64, t6225: f64, t464: f64, t7501: f64, t477: f64, t2489: f64, t5305: f64, t10720: f64, t10727: f64, t17547: f64, t17550: f64, t20759: f64, t20762: f64, t20764: f64, t20767: f64, t20768: f64) -> (f64, f64, f64, f64) {
    let t20773 = t132 * t137 * t2106 * t6225 / 10.0_f64;
    let t20774 = t7501 * t464;
    let t20778 = t132 * t137 * t20774 * t477 / 30.0_f64;
    let t20780 = 2.0_f64 / 15.0_f64 * t5305 * t2489;
    let t20781 = 0.6492624817418906_f64 * t17547 + 0.03354522822333102_f64 * t17550 + t20759 - t20762 + t20764 + t20767 - t20768 + 4.0_f64 / 3.0_f64 * t10720 + t10727 - t20773 - t20778 - t20780;
    (t20773, t20778, t20780, t20781)
}
