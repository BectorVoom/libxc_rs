//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1306/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1306<F: Float>(t213: F, t5527: F, t221: F, t776: F, t4119: F, t4128: F, t12986: F, t13002: F, t13005: F, t13010: F, t16769: F, t4127: F, t9526: F, t9540: F, t9542: F, t9547: F, t9572: F) -> F {
    let t16771 = t213 * t5527;
    let t16773 = t221 * t16771 * t776;
    let t16777 = t221 * t4128 * t4119;
    let t16781 = F::cast_from(0.16666666666666666666e-2_f64) * t9526 - t9540 - F::cast_from(0.12962962962962962963e-1_f64) * t9542 - F::cast_from(0.52777777777777777776e-2_f64) * t9547 + F::cast_from(0.33333333333333333332e-2_f64) * t12986 - t13002 - t9572 - F::cast_from(0.11666666666666666666e-1_f64) * t16769 - F::cast_from(0.19999999999999999999e-1_f64) * t13005 * t16773 + F::cast_from(0.99999999999999999996e-2_f64) * t4127 * t16777 - F::cast_from(0.25925925925925925925e-1_f64) * t13010;
    t16781
}
