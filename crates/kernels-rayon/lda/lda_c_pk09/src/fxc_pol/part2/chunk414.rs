//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 414/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk414(t1826: f64, t1924: f64, t1978: f64, t2129: f64, t417: f64, t1130: f64, t1135: f64, t1658: f64, t1663: f64, t564: f64, t560: f64, t561: f64) -> (f64, f64, f64) {
    let t2131 = t1826 + t1924 + t1978 + t2129;
    let t2132 = t417 * t2131;
    let t2134 = t564 / 4.0_f64 + t1130 / 4.0_f64 + t1135 / 8.0_f64 + t1658 / 8.0_f64 + t1663 / 8.0_f64 + t2132 / 8.0_f64;
    let t2137 = 2.0_f64 * t560 + 2.0_f64 * t561;
    (t2131, t2134, t2137)
}
