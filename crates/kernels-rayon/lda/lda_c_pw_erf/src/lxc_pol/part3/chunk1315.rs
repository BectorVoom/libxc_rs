//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1315/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1315(t5791: f64, t656: f64, t3912: f64, t5795: f64, t5798: f64, t2260: f64, t3915: f64, t1217: f64, t2281: f64, t3704: f64, t858: f64, t14108: f64, t14112: f64, t14157: f64, t14162: f64, t14164: f64, t14166: f64, t14170: f64) -> f64 {
    let t15143 = t5791 * t656;
    let t15144 = 4.0_f64 / 3.0_f64 * t15143;
    let t15145 = t5795 * t3912;
    let t15146 = (2e-21_f64 as f64) * t15145;
    let t15147 = t5798 * t656;
    let t15149 = t2260 * t3915;
    let t15150 = (2e-21_f64 as f64) * t15149;
    let t15151 = t2281 * t1217;
    let t15152 = 2.0_f64 / 45.0_f64 * t15151;
    let t15153 = t858 * t3704;
    let t15155 = t15144 + t15146 + 2.0_f64 / 3.0_f64 * t15147 + t15150 + t14108 + t14112 + t15152 - 8.0_f64 / 405.0_f64 * t15153 + t14157 - t14162 + t14164 + t14166 + t14170;
    t15155
}
