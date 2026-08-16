//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 823/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk823(t8276: f64, t8286: f64, t8298: f64, t8313: f64, t2254: f64, t3166: f64, t633: f64, t2246: f64, t650: f64, t896: f64, t2258: f64, t694: f64, t903: f64) -> (f64, f64, f64, f64) {
    let t8315 = t8276 + t8286 + t8298 + t8313;
    let t8318 = t3166 * t2254 * t633;
    let t8322 = t896 * t2246 * t650;
    let t8326 = t903 * t2258 * t694;
    (t8315, t8318, t8322, t8326)
}
