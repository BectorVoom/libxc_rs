//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 588/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk588(t4281: f64, t90: f64, t1106: f64, t4361: f64, t4093: f64, t623: f64, t896: f64, t1063: f64, t4365: f64, t10: f64, t104: f64, t125: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4480 = 5.0_f64 / 27.0_f64 * t90 * t4281;
    let t4489 = t1106 * t4361;
    let t4494 = t896 * t4093 * t623;
    let t4497 = t1063 * t4361;
    let t4499 = t1063 * t4365;
    let t4502 = t104 * t125 * t10;
    (t4480, t4489, t4494, t4497, t4499, t4502)
}
