//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1001/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1001(t518: f64, t6579: f64, t1450: f64, t6988: f64, t352: f64, t743: f64, t4738: f64, t5310: f64, t1318: f64, t4794: f64, t6370: f64, t172: f64, t184: f64, t6629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15694 = t6579 * t518;
    let t15697 = t6988 * t1450;
    let t15727 = t743 * t352;
    let t15743 = t4738 * t5310;
    let t15750 = t1318 * t4794 * t6370;
    let t15761 = t172 * t6629 * t184;
    (t15694, t15697, t15727, t15743, t15750, t15761)
}
