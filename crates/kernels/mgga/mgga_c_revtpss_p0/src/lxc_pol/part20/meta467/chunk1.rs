//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1787/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1787<F: Float>(t9303: F, t9641: F, t2435: F, t9635: F, t213: F, t225: F, t4071: F, t47343: F, t47568: F, t47570: F, t47574: F, t47580: F, t47591: F, t47593: F, t47595: F, t47601: F, t47606: F, t47608: F, t47612: F, t47616: F, t561: F, t9652: F) -> F {
    let t47618 = t9303 * t9641;
    let t47620 = t2435 * t9635;
    let t47622 = F::cast_from(0.44178176337912614788e-3_f64) * t47568 - F::cast_from(0.18505311230957427423e-1_f64) * t47570 - F::cast_from(0.78548797528808629095e-3_f64) * t47574 + F::cast_from(0.15805078039045227836e2_f64) * t4071 * t9652 - F::cast_from(0.1561190486301245283e0_f64) * t47580 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t47343 * t225 * t561 - t47591 + F::cast_from(0.65854491829355115985e-1_f64) * t47593 - F::cast_from(0.43902994552903410657e-1_f64) * t47595 + t47601 - F::cast_from(0.23417857294518679245e0_f64) * t47606 + F::cast_from(0.87805989105806821314e-1_f64) * t47608 + F::cast_from(0.23417857294518679246e0_f64) * t47612 - F::cast_from(0.39029762157531132075e-2_f64) * t47616 + F::cast_from(0.1040793657534163522e-1_f64) * t47618 + F::cast_from(0.43902994552903410657e-1_f64) * t47620;
    t47622
}
