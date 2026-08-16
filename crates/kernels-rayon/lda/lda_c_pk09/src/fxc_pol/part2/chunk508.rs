//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 508/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk508(t1755: f64, t1766: f64, t1771: f64, t1773: f64, t2733: f64, t2736: f64, t2803: f64, t2807: f64, t1776: f64, t452: f64, t2730: f64, t537: f64) -> (f64, f64, f64, f64) {
    let t2938 = t1755 - 6.25_f64 * t2803 + t1766 + 6.25_f64 * t2807 + t1771 - 1.2466946262544771_f64 * t2733 + t1773 + 1.2466946262544771_f64 * t2736;
    let t2939 = t2938 * t1776;
    let t2940 = t2939 * t452;
    let t2943 = t537 * t2730;
    (t2938, t2939, t2940, t2943)
}
