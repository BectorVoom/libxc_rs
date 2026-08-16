//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 275/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk275<F: Float>(t241: F, t252: F, t776: F, t802: F, t805: F, t810: F, t819: F, t825: F, t829: F, t838: F, t256: F) -> (F, F, F) {
    let t842 = t241 * (-F::cast_from(0.3109e-1_f64) * t805 * t252 + F::cast_from(1.0_f64) * t810 * t819 + t776 - t802 - F::cast_from(0.19751789702565206229e-1_f64) * t825 + F::cast_from(0.58482233974552040708e0_f64) * t829 * t838);
    let t844 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t825;
    let t845 = t241 * t256;
    (t842, t844, t845)
}
