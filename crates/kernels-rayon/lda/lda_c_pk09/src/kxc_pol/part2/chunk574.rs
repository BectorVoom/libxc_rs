//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 574/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk574(t160: f64, t2971: f64, t701: f64, t183: f64, t699: f64, t3930: f64, t698: f64, t655: f64, t694: f64, t121: f64, t120: f64, t718: f64) -> (f64, f64, f64, f64) {
    let t4044 = t160 * t2971;
    let t4049 = t701 * t701;
    let t4050 = 1.0_f64 / t4049;
    let t4053 = t183 * t699;
    let t4056 = 1.0_f64 / t3930;
    let t4057 = t698 * t4056;
    let t4059 = -2.0_f64 * t4053 * t655 + t4057 * t694;
    let t4060 = t121 * t4059;
    let t4061 = t120 * t4060;
    let t4064 = t718 * t2971;
    (t4044, t4050, t4061, t4064)
}
