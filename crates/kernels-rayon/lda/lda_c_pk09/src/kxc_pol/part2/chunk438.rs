//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 438/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk438(t169: f64, t2149: f64, t849: f64, t2159: f64, t2163: f64, t2167: f64, t2171: f64, t2175: f64, t2179: f64, t813: f64, t814: f64, t818: f64, t819: f64) -> (f64, f64) {
    let t2279 = t849 * t169 * t2149;
    let t2288 = t813 + t814 + 12.0_f64 * t2159 + 12.0_f64 * t2163 - 12.0_f64 * t2167 + t818 + t819 + 0.821419393556371_f64 * t2171 + 0.821419393556371_f64 * t2175 - 0.821419393556371_f64 * t2179;
    (t2279, t2288)
}
