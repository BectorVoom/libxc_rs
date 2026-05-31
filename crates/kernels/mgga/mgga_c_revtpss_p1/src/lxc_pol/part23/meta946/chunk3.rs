//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3119/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3119<F: Float>(t1179: F, t24252: F, t20641: F, t57854: F, t45232: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F) -> (F, F, F) {
    let t82050 = t24252 * t1179;
    let t82060 = F::cast_from(18.0_f64) * t57854 * t20641;
    let t82093 = F::cast_from(0.22831111111111111111e-1_f64) * t68255 - F::cast_from(0.1522074074074074074e-1_f64) * t68257 + F::cast_from(0.11415555555555555555e-1_f64) * t81156 - F::cast_from(0.34246666666666666667e-1_f64) * t81158 + F::cast_from(0.57077777777777777775e-1_f64) * t81162 + F::cast_from(0.2283111111111111111e0_f64) * t81167 + t45232 - F::cast_from(0.20547999999999999999e0_f64) * t81171 - F::cast_from(0.41095999999999999999e0_f64) * t81175 - F::cast_from(0.34246666666666666665e-1_f64) * t81179 - F::cast_from(0.11415555555555555555e-1_f64) * t81184 - F::cast_from(0.34246666666666666665e-1_f64) * t81188 + F::cast_from(0.30822e0_f64) * t81192 + F::cast_from(0.41096e0_f64) * t81196 + F::cast_from(0.10274e0_f64) * t81200 + F::cast_from(0.10274e0_f64) * t81204 + F::cast_from(0.34246666666666666666e-1_f64) * t81209 - F::cast_from(0.50735802469135802467e-1_f64) * t81214 - F::cast_from(0.19025925925925925925e-1_f64) * t68262 - F::cast_from(0.34246666666666666666e-1_f64) * t68277;
    (t82050, t82060, t82093)
}
