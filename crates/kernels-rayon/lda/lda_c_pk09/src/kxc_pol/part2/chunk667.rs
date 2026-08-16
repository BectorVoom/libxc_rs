//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 667/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk667(t1487: f64, t4998: f64, t1416: f64, t337: f64, t1387: f64, t401: f64, t1837: f64, t633: f64, t333: f64) -> (f64, f64, f64, f64) {
    let t6164 = 1.6183441301295518_f64 * t1487 * t4998;
    let t6167 = t1416 * t337;
    let t6168 = t6167 * t1387;
    let t6174 = t401 * t401;
    let t6175 = 1.0_f64 / t6174;
    let t6195 = t1837 * t633;
    let t6196 = t333 * t6195;
    (t6164, t6168, t6175, t6196)
}
