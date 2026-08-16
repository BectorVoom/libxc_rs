//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 698/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk698(t1948: f64, t6620: f64, t1927: f64, t6488: f64, t1901: f64, t6477: f64, t490: f64, t6601: f64, t508: f64, t6501: f64, t6505: f64, t6508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6622 = 0.027433775686566395_f64 * t1948 * t6620;
    let t6624 = 12.423505345088643_f64 * t1927 * t6488;
    let t6625 = t1901 * t6477;
    let t6628 = 1.6715885419444727_f64 * t490 * t6601;
    let t6630 = 2.1943705410881575_f64 * t508 * t6601;
    let t6633 = 2.0_f64 * t6501;
    let t6634 = 2.0_f64 * t6505;
    let t6635 = 2.6666666666666665_f64 * t6508;
    (t6622, t6624, t6625, t6628, t6630, t6633, t6634, t6635)
}
