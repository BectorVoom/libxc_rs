//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 410/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk410(t1672: f64, t472: f64, t453: f64, t1971: f64, t471: f64, t1782: f64, t1985: f64) -> (f64, f64, f64, f64) {
    let t2108 = t472 * t1672 / 18.0_f64;
    let t2110 = t453 * t1672 / 18.0_f64;
    let t2111 = t471 * t1971;
    let t2114 = t1985 * t1782;
    (t2108, t2110, t2111, t2114)
}
