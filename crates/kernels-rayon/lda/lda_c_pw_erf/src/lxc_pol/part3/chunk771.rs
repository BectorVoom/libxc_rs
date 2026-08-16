//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 771/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk771(t181: f64, t944: f64, t184: f64, t786: f64, t494: f64, t509: f64, t2095: f64, t5021: f64, t803: f64, t933: f64, t1268: f64, t4615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5064 = t944 * t181;
    let t5065 = t5064 * t184;
    let t5067 = 4.0_f64 / 15.0_f64 * t5065 * t786;
    let t5068 = t494 * t509;
    let t5069 = t5068 * t184;
    let t5071 = 8.0_f64 / 15.0_f64 * t5069 * t786;
    let t5072 = t5021 * t2095;
    let t5076 = t933 * t803;
    let t5084 = t1268 * t4615;
    (t5064, t5065, t5067, t5068, t5069, t5071, t5072, t5076, t5084)
}
