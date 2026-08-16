//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2608/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2608(t14238: f64, t2453: f64, t10142: f64, t10019: f64, t14239: f64, t1882: f64, t4066: f64, t1398: f64, t21990: f64, t10022: f64, t2782: f64, t46463: f64, t46465: f64, t47995: f64, t47999: f64, t48004: f64, t48005: f64, t5675: f64, t5745: f64, t5767: f64, t820: f64, t9891: f64) -> (f64, f64) {
    let t48007 = t2453 * t14238;
    let t48008 = t48007 * t10142;
    let t48009 = 0.34697458558045176417e-2_f64 * t48008;
    let t48013 = t14239 * t10019;
    let t48015 = t4066 * t1882;
    let t48020 = t21990 * t1398;
    let t48022 = t2782 * t10022 * t48020;
    let t48024 = -0.91069445034239308175e-1_f64 * t46463 - 0.58544643236296698113e-1_f64 * t47995 - 0.29272321618148349057e-1_f64 * t47999 - t48004 + 0.26019841438354088051e-2_f64 * t48005 - t48009 - 0.65854491829355115987e0_f64 * t820 * t5767 * t9891 - 0.29272321618148349057e-1_f64 * t48013 + 0.39512695097613069591e1_f64 * t5745 * t48015 * t5675 + 0.19514881078765566037e-2_f64 * t46465 - 0.65854491829355115984e-1_f64 * t48022;
    (t48015, t48024)
}
