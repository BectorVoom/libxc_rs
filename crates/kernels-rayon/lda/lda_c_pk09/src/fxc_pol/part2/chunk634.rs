//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 634/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk634(t5293: f64, t93: f64, t1470: f64, t1403: f64, t4979: f64, t1240: f64, t1364: f64, t310: f64, t1337: f64, t1471: f64, t623: f64, t333: f64) -> (f64, f64, f64, f64, f64) {
    let t5294 = t93 * t5293;
    let t5296 = 7.108175748183851_f64 * t1470 * t5294;
    let t5298 = 2.2140749178833072_f64 * t1403 * t4979;
    let t5303 = t1364 * t1240;
    let t5304 = t310 * t5303;
    let t5305 = t1337 * t5304;
    let t5307 = t1471 * t623;
    let t5308 = t333 * t5307;
    (t5294, t5296, t5298, t5305, t5308)
}
