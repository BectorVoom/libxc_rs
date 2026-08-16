//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1154/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1154(t184: f64, t203: f64, t221: f64, t48341: f64, t48354: f64, t1044: f64, t12505: f64, t1815: f64, t639: f64, t12350: f64, t1620: f64, t1809: f64, t7452: f64) -> (f64, f64, f64) {
    let t48359 = 2.0_f64 / 15.0_f64 * t203 * (t48341 + t48354) * t184 * t221;
    let t48363 = 32.0_f64 / 15.0_f64 * t639 * t1815 * t12505 * t1044;
    let t48367 = 64.0_f64 / 15.0_f64 * t1620 * t1809 * t7452 * t12350;
    (t48359, t48363, t48367)
}
