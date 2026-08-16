//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1145/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1145(t12350: f64, t1620: f64, t23207: f64, t2677: f64, t12752: f64, t2612: f64, t1815: f64, t3469: f64, t3553: f64, t639: f64, t1044: f64, t12497: f64, t5522: f64) -> (f64, f64, f64, f64) {
    let t48213 = 64.0_f64 / 9.0_f64 * t1620 * t2677 * t23207 * t12350;
    let t48215 = 32.0_f64 / 15.0_f64 * t2612 * t12752;
    let t48219 = 16.0_f64 / 15.0_f64 * t639 * t1815 * t3469 * t3553;
    let t48223 = 32.0_f64 / 9.0_f64 * t639 * t5522 * t12497 * t1044;
    (t48213, t48215, t48219, t48223)
}
