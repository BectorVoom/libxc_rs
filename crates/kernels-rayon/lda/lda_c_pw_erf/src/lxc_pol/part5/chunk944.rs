//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 944/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk944(t1064: f64, t1799: f64, t285: f64, t4422: f64, t477: f64, t1128: f64, t1896: f64, t343: f64, t780: f64, t159: f64, t4437: f64, t2783: f64, t872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11471 = t1064 * t1799;
    let t11472 = 60.0_f64 * t11471;
    let t11498 = t4422 * t477 * t285;
    let t11499 = 0.0017434044910732151_f64 * t11498;
    let t11501 = t1896 * t1128 * t285;
    let t11546 = t343 * t780;
    let t11548 = t11546 * t159 * t285;
    let t11551 = t4437 * t477 * t285;
    let t11557 = t2783 * t872;
    (t11472, t11499, t11501, t11546, t11548, t11551, t11557)
}
