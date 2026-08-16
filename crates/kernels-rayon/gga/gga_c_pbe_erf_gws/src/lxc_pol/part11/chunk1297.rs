//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1297/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1297(t13070: f64, t13073: f64, t13088: f64, t13702: f64, t13704: f64, t16329: f64, t16331: f64, t16334: f64, t16335: f64, t16336: f64, t16337: f64, t16338: f64, t16340: f64, t48936: f64, t48948: f64, t48950: f64, t48957: f64, t50751: f64, t50759: f64, t50767: f64, t50771: f64, t6906: f64, t6968: f64, t7: f64, t9763: f64) -> f64 {
    let t50784 = t7 * (t48936 + t48948 + t48950 + t48957 + t50751 + t50759 + t50767 + t50771) - 4.0_f64 * t13070 - 0.21973866044103791929e-2_f64 * t9763 + 0.82152657680133333336e1_f64 * t6906 + t16329 - t16331 - t16334 + t16335 - t16336 - 12.0_f64 * t13073 - t16337 + t16338 + t16340 + 12.0_f64 * t13088 + 4.0_f64 * t13702 + 36.0_f64 * t13704 + 0.13012297059337829058e0_f64 * t6968;
    t50784
}
