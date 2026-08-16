//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1112/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1112(t34: f64, t6723: f64, t13771: f64, t4522: f64, t17645: f64, t2034: f64, t3974: f64, t16612: f64, t2010: f64, t4506: f64, t20688: f64, t20689: f64, t20691: f64, t20693: f64, t20695: f64, t20700: f64, t20704: f64, t20707: f64, t20710: f64, t20715: f64) -> (f64, f64, f64, f64, f64) {
    let t20716 = t6723 * t34;
    let t20719 = 16.0_f64 / 9.0_f64 * t13771 * t4522 * t20716;
    let t20722 = 16.0_f64 / 15.0_f64 * t3974 * t17645 * t2034;
    let t20725 = 8.0_f64 / 5.0_f64 * t4506 * t16612 * t2010;
    let t20726 = t20688 + t20689 - t20691 - t20693 + t20695 - t20700 - t20704 + t20707 - t20710 - t20715 + t20719 + t20722 - t20725;
    (t20716, t20719, t20722, t20725, t20726)
}
