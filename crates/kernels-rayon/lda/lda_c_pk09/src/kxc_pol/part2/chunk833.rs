//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 833/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk833(t169: f64, t7704: f64, t849: f64, t2149: f64, t4710: f64, t707: f64, t2288: f64, t825: f64, t609: f64, t121: f64, t4037: f64, t623: f64) -> (f64, f64, f64, f64, f64) {
    let t8475 = t849 * t169 * t7704;
    let t8484 = t4710 * t2149;
    let t8485 = t707 * t8484;
    let t8488 = t2288 * t825;
    let t8489 = t8488 * t609;
    let t8490 = t121 * t8489;
    let t8491 = t4037 * t8490;
    let t8493 = t8488 * t623;
    (t8475, t8485, t8488, t8491, t8493)
}
