//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 867/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk867(t4095: f64, t8990: f64, t1101: f64, t3105: f64, t3107: f64, t4085: f64, t4096: f64, t4109: f64, t4111: f64, t4123: f64, t4125: f64, t4128: f64, t709: f64, t8096: f64, t8101: f64, t8977: f64, t8980: f64, t8987: f64) -> f64 {
    let t8991 = t8990 * t4095;
    let t8998 = -t8977 * t709 / 6.0_f64 - t8980 / 6.0_f64 - t1101 * t8096 / 6.0_f64 - t1101 * t8101 / 6.0_f64 + t4085 * t8987 / 36.0_f64 - t8991 / 18.0_f64 - t4096 / 36.0_f64 - t4109 + t4111 / 6.0_f64 + t4123 / 6.0_f64 - t4125 - t4128 + 0.016445729887122652_f64 * t3105 + 0.016445729887122652_f64 * t3107;
    t8998
}
