//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 795/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk795(t2237: f64, t4050: f64, t119: f64, t4064: f64, t7731: f64, t2318: f64, t609: f64, t96: f64, t839: f64, t2213: f64, t572: f64) -> (f64, f64, f64, f64) {
    let t7973 = t2237 * t4050;
    let t7974 = t7973 * t119;
    let t7981 = t4064 * t7731;
    let t7988 = t96 * t2318 * t609;
    let t7989 = t839 * t7988;
    let t7991 = t572 * t2213;
    (t7974, t7981, t7989, t7991)
}
