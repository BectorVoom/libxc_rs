//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3119/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3119(t1179: f64, t24252: f64, t20641: f64, t57854: f64, t45232: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> (f64, f64, f64) {
    let t82050 = t24252 * t1179;
    let t82060 = 18.0_f64 * t57854 * t20641;
    let t82093 = 0.22831111111111111111e-1_f64 * t68255 - 0.1522074074074074074e-1_f64 * t68257 + 0.11415555555555555555e-1_f64 * t81156 - 0.34246666666666666667e-1_f64 * t81158 + 0.57077777777777777775e-1_f64 * t81162 + 0.2283111111111111111e0_f64 * t81167 + t45232 - 0.20547999999999999999e0_f64 * t81171 - 0.41095999999999999999e0_f64 * t81175 - 0.34246666666666666665e-1_f64 * t81179 - 0.11415555555555555555e-1_f64 * t81184 - 0.34246666666666666665e-1_f64 * t81188 + 0.30822e0_f64 * t81192 + 0.41096e0_f64 * t81196 + 0.10274e0_f64 * t81200 + 0.10274e0_f64 * t81204 + 0.34246666666666666666e-1_f64 * t81209 - 0.50735802469135802467e-1_f64 * t81214 - 0.19025925925925925925e-1_f64 * t68262 - 0.34246666666666666666e-1_f64 * t68277;
    (t82050, t82060, t82093)
}
