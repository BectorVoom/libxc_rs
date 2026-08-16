//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 420/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk420(t2152: f64, t95: f64, t120: f64, t119: f64, t2143: f64, t63: f64, t673: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2153 = t95 * t2152;
    let t2154 = t120 * t2153;
    let t2155 = t119 * t2154;
    let t2157 = t63 * t2143;
    let t2158 = t673 * t2157;
    let t2159 = t672 * t2158;
    (t2153, t2154, t2155, t2157, t2158, t2159)
}
