//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1149/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1149(t1809: f64, t47987: f64, t639: f64, t1815: f64, t3473: f64, t3553: f64, t1620: f64, t2677: f64, t3465: f64, t3562: f64, t1022: f64, t12493: f64, t7853: f64) -> (f64, f64, f64, f64) {
    let t48291 = 32.0_f64 / 45.0_f64 * t639 * t1809 * t47987;
    let t48295 = 8.0_f64 / 15.0_f64 * t639 * t1815 * t3473 * t3553;
    let t48299 = 16.0_f64 / 9.0_f64 * t1620 * t2677 * t3465 * t3562;
    let t48303 = 256.0_f64 / 81.0_f64 * t1620 * t7853 * t12493 * t1022;
    (t48291, t48295, t48299, t48303)
}
