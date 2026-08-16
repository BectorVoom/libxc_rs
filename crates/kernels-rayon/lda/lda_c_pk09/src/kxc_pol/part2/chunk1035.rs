//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1035/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1035(t2149: f64, t633: f64, t6938: f64, t1904: f64, t11068: f64, t6814: f64, t6818: f64, t142: f64, t480: f64, t4007: f64, t92: f64, t1240: f64) -> (f64, f64, f64, f64, f64) {
    let t11206 = t2149 * t633;
    let t11207 = t6938 * t11206;
    let t11208 = t1904 * t11207;
    let t11211 = t6818 * t6814 * t11068;
    let t11213 = t480 * t142;
    let t11214 = t92 * t4007;
    let t11216 = t11213 * t11214 * t11206;
    let t11218 = t2149 * t1240;
    (t11206, t11208, t11211, t11216, t11218)
}
