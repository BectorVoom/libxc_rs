//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1209/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1209(t10714: f64, t13096: f64, t13097: f64, t13099: f64, t13103: f64, t13106: f64, t13108: f64, t13112: f64, t13114: f64, t13116: f64, t13118: f64, t13120: f64, t13123: f64, t13125: f64, t13128: f64, t13133: f64, t13134: f64, t13138: f64, t13140: f64, t13144: f64, t13149: f64, t13151: f64, t13153: f64) -> (f64, f64) {
    let t14409 = t10714 + t13096 - t13097 + t13099 + t13103 + t13106 + t13108 + t13112 + t13114 + t13116 + t13118;
    let t14410 = t13120 + t13123 - t13125 - t13128 + t13133 - t13134 - t13138 - t13140 - t13144 + t13149 + t13151 + t13153;
    (t14409, t14410)
}
