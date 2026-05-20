//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2608/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2608<F: Float>(t14238: F, t2453: F, t10142: F, t10019: F, t14239: F, t1882: F, t4066: F, t1398: F, t21990: F, t10022: F, t2782: F, t46463: F, t46465: F, t47995: F, t47999: F, t48004: F, t48005: F, t5675: F, t5745: F, t5767: F, t820: F, t9891: F) -> (F, F) {
    let t48007 = t2453 * t14238;
    let t48008 = t48007 * t10142;
    let t48009 = F::cast_from(0.34697458558045176417e-2_f64) * t48008;
    let t48013 = t14239 * t10019;
    let t48015 = t4066 * t1882;
    let t48020 = t21990 * t1398;
    let t48022 = t2782 * t10022 * t48020;
    let t48024 = -F::cast_from(0.91069445034239308175e-1_f64) * t46463 - F::cast_from(0.58544643236296698113e-1_f64) * t47995 - F::cast_from(0.29272321618148349057e-1_f64) * t47999 - t48004 + F::cast_from(0.26019841438354088051e-2_f64) * t48005 - t48009 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t5767 * t9891 - F::cast_from(0.29272321618148349057e-1_f64) * t48013 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t48015 * t5675 + F::cast_from(0.19514881078765566037e-2_f64) * t46465 - F::cast_from(0.65854491829355115984e-1_f64) * t48022;
    (t48015, t48024)
}
