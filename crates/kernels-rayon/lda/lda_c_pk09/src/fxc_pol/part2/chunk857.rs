//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 857/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk857(t2275: f64, t748: f64, t2279: f64, t62: f64, t7704: f64, t903: f64, t890: f64, t2152: f64, t650: f64, t891: f64, t61: f64, t7831: f64) -> (f64, f64, f64, f64, f64) {
    let t8849 = t748 * t2275;
    let t8851 = t748 * t2279;
    let t8857 = t62 * t7704;
    let t8858 = t903 * t8857;
    let t8859 = t890 * t8858;
    let t8861 = t650 * t2152;
    let t8862 = t891 * t8861;
    let t8863 = t890 * t8862;
    let t8865 = t61 * t7831;
    (t8849, t8851, t8859, t8863, t8865)
}
