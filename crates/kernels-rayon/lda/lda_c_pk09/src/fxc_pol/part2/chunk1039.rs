//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1039/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1039(t1675: f64, t2759: f64, t1672: f64, t2755: f64, t11101: f64, t1797: f64, t1800: f64, t2901: f64, t6230: f64, t452: f64, t1947: f64, t2902: f64) -> (f64, f64, f64, f64, f64) {
    let t11262 = t2759 * t1675;
    let t11264 = t2755 * t1672;
    let t11270 = t1797 * t11101;
    let t11271 = t11270 * t1800;
    let t11273 = t2901 * t6230;
    let t11274 = t11273 * t452;
    let t11277 = t2902 * t1947;
    (t11262, t11264, t11271, t11274, t11277)
}
