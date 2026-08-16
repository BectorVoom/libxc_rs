//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1009/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1009(t1980: f64, t35500: f64, t7476: f64, t31262: f64, t31277: f64, t31279: f64, t1988: f64, t8486: f64, t1967: f64, t8838: f64, t31285: f64, t4360: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35502 = t1980 * t7476 * t35500;
    let t35506 = 0.26147916666666666666e0_f64 * t31262;
    let t35507 = 0.3973125e0_f64 * t31277;
    let t35508 = 0.264875e0_f64 * t31279;
    let t35513 = t1988 * t8486;
    let t35515 = t1967 * t8838;
    let t35527 = 0.10718504529517434243e-2_f64 * t31285;
    let t35529 = t7741 * t4360;
    (t35502, t35506, t35507, t35508, t35513, t35515, t35527, t35529)
}
