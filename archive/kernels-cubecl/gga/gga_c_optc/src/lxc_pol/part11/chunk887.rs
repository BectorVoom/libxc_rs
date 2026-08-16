//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 887/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk887<F: Float>(t16741: F, t16769: F, t818: F, t16708: F, t7801: F, t10478: F, t1355: F, t1367: F, t14102: F, t14235: F, t16654: F, t16657: F, t16672: F, t16676: F, t16677: F, t16680: F, t16683: F, t16686: F, t16699: F, t16709: F, t2493: F, t2518: F, t252: F, t2530: F, t2537: F, t3716: F, t4885: F, t4888: F, t7759: F, t7799: F, t810: F) -> (F, F, F, F) {
    let t16770 = t16741 + t16769;
    let t16771 = t16770 * t818;
    let t16774 = t16708 * t7801;
    let t16779 = t16654 + t16657 - F::cast_from(0.19751789702565206229e-1_f64) * t16672 - t16676 - F::cast_from(6.0_f64) * t2493 * t16677 + F::cast_from(0.96494049533612093922e2_f64) * t2518 * t16680 - F::cast_from(0.35089340384731224426e1_f64) * t2530 * t16683 + F::cast_from(0.51947267698127589897e2_f64) * t2537 * t16686 - F::cast_from(0.3109e-1_f64) * t16699 * t252 + F::cast_from(3.0_f64) * t14235 * t1355 + F::cast_from(3.0_f64) * t3716 * t4885 + F::cast_from(0.96494049533612093922e2_f64) * t10478 * t4888 - F::cast_from(0.19298809906722418785e3_f64) * t7759 * t16709 + F::cast_from(1.0_f64) * t810 * t16771 + F::cast_from(0.20691336878655965246e4_f64) * t7799 * t16774 + F::cast_from(0.17544670192365612213e1_f64) * t14102 * t1367;
    (t16770, t16771, t16774, t16779)
}
