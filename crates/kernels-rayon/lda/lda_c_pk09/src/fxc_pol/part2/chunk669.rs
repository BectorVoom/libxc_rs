//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 669/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk669(t454: f64, t6223: f64, t1948: f64, t1927: f64, t6196: f64, t1895: f64, t1893: f64, t529: f64, t532: f64, t1892: f64, t1792: f64, t1884: f64) -> (f64, f64, f64, f64, f64) {
    let t6224 = t454 * t6223;
    let t6225 = t1948 * t6224;
    let t6227 = t1927 * t6196;
    let t6229 = t1895 * t1895;
    let t6230 = 1.0_f64 / t6229;
    let t6233 = t529 * t1893;
    let t6236 = t532 * t532;
    let t6237 = 1.0_f64 / t6236;
    let t6238 = t1892 * t6237;
    let t6240 = t1792 * t6238 - 2.0_f64 * t1884 * t6233;
    (t6225, t6227, t6230, t6236, t6240)
}
