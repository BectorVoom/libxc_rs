//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 864/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk864(t8911: f64, t8918: f64, t8927: f64, t8942: f64, t61: f64, t825: f64, t96: f64, t2143: f64, t844: f64, t873: f64, t2251: f64, t748: f64) -> (f64, f64, f64) {
    let t8944 = t8911 + t8918 + t8927 + t8942;
    let t8947 = t96 * t61 * t8944 * t825;
    let t8953 = t844 * t873 * t2143;
    let t8964 = t748 * t2251;
    (t8947, t8953, t8964)
}
