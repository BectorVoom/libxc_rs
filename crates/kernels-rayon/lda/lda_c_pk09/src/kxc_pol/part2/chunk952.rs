//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 952/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk952(t10020: f64, t5683: f64, t318: f64, t332: f64, t2520: f64, t623: f64, t333: f64) -> (f64, f64, f64) {
    let t10021 = t5683 * t10020;
    let t10023 = t318 * t332;
    let t10024 = t2520 * t623;
    let t10025 = t333 * t10024;
    (t10021, t10023, t10025)
}
