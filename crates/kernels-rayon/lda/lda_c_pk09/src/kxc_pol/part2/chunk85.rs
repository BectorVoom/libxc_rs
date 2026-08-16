//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 85/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk85(t43: f64, t6: f64, t2: f64, t3: f64, t1: f64, t4: f64) -> (f64, f64, f64, f64, f64) {
    let t254 = 1.1801314654631911_f64 * t43;
    let t255 = 1.4269304149842164_f64 * t6;
    let t256 = t3 * t2;
    let t257 = t4 * t1;
    let t258 = 1.0_f64 / t257;
    (t254, t255, t256, t257, t258)
}
