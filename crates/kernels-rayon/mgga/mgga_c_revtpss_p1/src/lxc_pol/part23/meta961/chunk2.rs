//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3247/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3247(t85420: f64, t85440: f64, t1424: f64, t14299: f64, t22415: f64, t22971: f64, t23043: f64, t4071: f64, t4076: f64, t46359: f64, t47764: f64, t47772: f64, t47781: f64, t47785: f64, t47786: f64, t5715: f64, t5774: f64, t6896: f64, t6918: f64, t73587: f64, t73590: f64, t73593: f64, t73598: f64) -> (f64, f64) {
    let t85442 = t85420 / 2.0_f64 + t85440 / 2.0_f64;
    let t85466 = 0.58911598146606471821e-3_f64 * t47764 - 0.39029762157531132076e-1_f64 * t73587 + 0.16463622957338778996e-1_f64 * t73590 + 0.39029762157531132074e-2_f64 * t73593 + 0.29272321618148349057e-1_f64 * t73598 - 0.65854491829355115987e0_f64 * t4071 * t23043 + 0.39512695097613069591e1_f64 * t4071 * t22971 + 0.33133632253434461091e-3_f64 * t47772 + 0.39512695097613069592e1_f64 * t5715 * t22415 + 0.39512695097613069591e1_f64 * t14299 * t6896 - 0.58911598146606471821e-3_f64 * t47781 + 0.39512695097613069591e1_f64 * t1424 * t4076 * t5774 * t6918 + t47785 - 0.78059524315062264151e-2_f64 * t47786 + t46359;
    (t85442, t85466)
}
