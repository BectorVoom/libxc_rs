//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1271/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1271(t18415: f64, t1325: f64, t3859: f64, t7808: f64, t2171: f64, t6233: f64, t4738: f64, t6292: f64, t6230: f64, t1318: f64, t3854: f64, t7733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22821 = 32.0_f64 / 45.0_f64 * t18415;
    let t22823 = t1325 * t3859 * t7808;
    let t22824 = 16.0_f64 / 45.0_f64 * t22823;
    let t22825 = t2171 * t6233;
    let t22826 = 16.0_f64 / 45.0_f64 * t22825;
    let t22827 = t4738 * t6292;
    let t22828 = 32.0_f64 / 45.0_f64 * t22827;
    let t22830 = 16.0_f64 / 15.0_f64 * t4738 * t6230;
    let t22832 = t1318 * t3854 * t7733;
    (t22821, t22824, t22826, t22828, t22830, t22832)
}
