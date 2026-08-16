//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 787/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk787(t1175: f64, t4969: f64, t724: f64, t1168: f64, t5064: f64, t10052: f64, t242: f64, t10157: f64, t21416: f64, t265: f64, t3977: f64, t5147: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21524 = t724 * t1175 * t4969;
    let t21531 = t5064 * t1168;
    let t21532 = t10052 * t21531;
    let t21533 = t242 * t21532;
    let t21537 = t10157 * t265 * t21416;
    let t21540 = t3977 * t5147;
    (t21524, t21531, t21532, t21533, t21537, t21540)
}
