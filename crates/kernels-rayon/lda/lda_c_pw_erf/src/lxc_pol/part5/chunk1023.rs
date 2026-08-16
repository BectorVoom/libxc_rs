//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1023/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1023(t519: f64, t5237: f64, t6336: f64, t2494: f64, t933: f64, t331: f64, t6558: f64, t5021: f64, t6528: f64, t6519: f64, t6522: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17156 = t519 * t5237 * t6336;
    let t17226 = t933 * t2494;
    let t17234 = t331 * t6558;
    let t17249 = t5021 * t6528;
    let t17272 = t331 * t6519;
    let t17274 = t5021 * t6522;
    let t17288 = t331 * t6525;
    (t17156, t17226, t17234, t17249, t17272, t17274, t17288)
}
