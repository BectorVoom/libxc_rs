//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 967/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk967(t1614: f64, t9759: f64, t327: f64, t9819: f64, t1625: f64, t1610: f64, t2474: f64, t93: f64, t1336: f64, t2551: f64, t332: f64, t9836: f64) -> (f64, f64, f64, f64, f64) {
    let t10262 = t9759 * t1614;
    let t10269 = t327 * t9819;
    let t10270 = t10269 * t1625;
    let t10274 = t1610 * t2474;
    let t10275 = t93 * t10274;
    let t10280 = t2551 * t1336;
    let t10281 = t10280 * t1625;
    let t10287 = t332 * t9836;
    (t10262, t10270, t10275, t10281, t10287)
}
