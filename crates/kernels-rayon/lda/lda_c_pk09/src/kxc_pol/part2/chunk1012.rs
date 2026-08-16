//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1012/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1012(t2711: f64, t4785: f64, t1151: f64, t2704: f64, t1161: f64, t1156: f64, t4842: f64, t9648: f64, t9649: f64, t9650: f64, t9651: f64, t420: f64) -> (f64, f64, f64, f64) {
    let t10968 = t4785 * t2711;
    let t10974 = t1151 * t2704;
    let t10976 = t2704 * t1161;
    let t10977 = t1156 * t10976;
    let t10979 = t9648 + t9649 - t9650 - t9651 - t4842;
    let t10980 = t10979 * t420;
    (t10968, t10974, t10977, t10980)
}
