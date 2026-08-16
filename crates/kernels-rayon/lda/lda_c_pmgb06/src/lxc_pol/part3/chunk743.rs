//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 743/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk743(t5049: f64, t1547: f64, t814: f64, t132: f64, t2998: f64, t3007: f64, t4070: f64, t4079: f64, t4082: f64, t4089: f64, t4091: f64, t4973: f64, t4977: f64, t4981: f64, t4983: f64, t5043: f64, t5046: f64, t5048: f64) -> (f64, f64, f64, f64, f64) {
    let t5050 = t5049 / 135.0_f64;
    let t5051 = t1547 * t814;
    let t5052 = t132 * t5051;
    let t5053 = t5052 / 135.0_f64;
    let t5054 = 2.0_f64 / 45.0_f64 * t2998;
    let t5057 = -t4973 - t4977 - t4981 - t4983 - t5043 - t5046 - t5048 - t5050 - t5053 - t5054 + t3007 + t4070 + t4079 + t4082 + t4089 / 3.0_f64 + 0.06077777777777778_f64 * t4091;
    (t5050, t5051, t5053, t5054, t5057)
}
