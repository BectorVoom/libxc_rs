//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 683/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk683(t453: f64, t6189: f64, t1830: f64, t473: f64, t6185: f64, t6160: f64, t1619: f64, t6165: f64, t2571: f64, t350: f64, t2575: f64, t2579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6190 = t453 * t6189;
    let t6191 = t1830 * t6190;
    let t6193 = t473 * t6185;
    let t6196 = t473 * t6189;
    let t6199 = t473 * t6160;
    let t6202 = t1619 * t6165;
    let t6205 = t350 * t2571;
    let t6207 = t350 * t2575;
    let t6209 = t350 * t2579;
    (t6190, t6191, t6193, t6196, t6199, t6202, t6205, t6207, t6209)
}
