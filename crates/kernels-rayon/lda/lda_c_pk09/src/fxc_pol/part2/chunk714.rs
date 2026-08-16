//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 714/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk714(t453: f64, t7066: f64, t2053: f64, t6253: f64, t472: f64, t2045: f64, t7030: f64, t1947: f64, t2103: f64, t2042: f64, t2102: f64, t305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7067 = t453 * t7066;
    let t7069 = t2053 * t6253;
    let t7071 = t472 * t7066;
    let t7074 = t2045 * t7030 / 6.0_f64;
    let t7075 = t2103 * t1947;
    let t7076 = t7075 * t2042;
    let t7080 = t2102 * t305;
    (t7067, t7069, t7071, t7074, t7076, t7080)
}
