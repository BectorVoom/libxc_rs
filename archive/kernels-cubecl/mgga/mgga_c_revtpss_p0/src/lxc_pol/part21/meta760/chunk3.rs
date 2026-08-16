//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2687/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2687<F: Float>(t10130: F, t1399: F, t14122: F, t1437: F, t4004: F, t47442: F, t47444: F, t48438: F, t49280: F, t49399: F, t49403: F, t49407: F, t49426: F, t5659: F, t5745: F, t5755: F, t5767: F, t820: F, t9899: F) -> F {
    let t49428 = F::cast_from(0.17563392970889009434e0_f64) * t49399 + F::cast_from(0.58544643236296698112e-1_f64) * t49403 + t47442 + F::cast_from(0.29272321618148349057e-1_f64) * t49407 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t5767 * t9899 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t48438 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t49280 * t1399 + F::cast_from(0.11853808529283920877e2_f64) * t5745 * t14122 * t4004 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t10130 * t5659 + F::cast_from(0.91069445034239308175e-1_f64) * t47444 - F::cast_from(0.19514881078765566037e-2_f64) * t49426;
    t49428
}
