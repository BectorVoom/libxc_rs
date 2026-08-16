//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1322/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322<F: Float>(t13005: F, t16771: F, t20800: F, t210: F, t214: F, t221: F, t2571: F, t41155: F, t41161: F, t41185: F, t41200: F, t4127: F, t4128: F, t46764: F, t46772: F, t46790: F, t5544: F, t68073: F, t68110: F, t75978: F, t76056: F, t76063: F, t787: F) -> F {
    let t76359 = t41155 - t41185 - F::cast_from(0.11999999999999999999e0_f64) * t13005 * t221 * t16771 * t5544 + F::cast_from(0.19999999999999999999e-1_f64) * t4127 * t221 * t4128 * t20800 + F::cast_from(0.99999999999999999995e-1_f64) * t41161 * t210 * t214 * t76056 + F::cast_from(0.14999999999999999999e-1_f64) * t2571 * t210 * t214 * t76063 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t210 * t214 * t75978 - F::cast_from(0.79999999999999999997e-1_f64) * t46764 - F::cast_from(0.13999999999999999999e0_f64) * t68073 + F::cast_from(0.13148148148148148148e0_f64) * t46772 - t41200 - F::cast_from(0.29999999999999999998e-1_f64) * t68110 + F::cast_from(0.22469135802469135801e0_f64) * t46790;
    t76359
}
