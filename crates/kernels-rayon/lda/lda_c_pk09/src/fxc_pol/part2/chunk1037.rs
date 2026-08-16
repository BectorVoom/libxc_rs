//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1037/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1037(t1672: f64, t2946: f64, t1665: f64, t2759: f64, t1671: f64, t2913: f64, t1904: f64, t1222: f64, t2751: f64) -> (f64, f64, f64, f64) {
    let t11238 = t2946 * t1672;
    let t11243 = t2759 * t1665;
    let t11245 = t1671 * t2913;
    let t11246 = t1904 * t11245;
    let t11248 = t1222 * t2751;
    (t11238, t11243, t11246, t11248)
}
