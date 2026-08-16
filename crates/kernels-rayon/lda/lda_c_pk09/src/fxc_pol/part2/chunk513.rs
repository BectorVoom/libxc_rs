//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 513/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk513(t2972: f64, t2974: f64, t119: f64, t863: f64, t1062: f64, t721: f64, t572: f64, t755: f64) -> (f64, f64, f64, f64) {
    let t2975 = t2972 * t2974;
    let t2977 = t863 * t119;
    let t2980 = t863 * t1062;
    let t2981 = t2980 * t721;
    let t2983 = t572 * t755;
    (t2975, t2977, t2981, t2983)
}
