//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1298/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1298(t10017: f64, t12363: f64, t13708: f64, t13711: f64, t13714: f64, t13717: f64, t13721: f64, t16345: f64, t16349: f64, t16350: f64, t16351: f64, t16353: f64, t16354: f64, t7986: f64, t7988: f64, t7990: f64, t8520: f64) -> f64 {
    let t50796 = 24.0_f64 * t13708 + 36.0_f64 * t13711 + 72.0_f64 * t13714 - 0.75926915593978166528e1_f64 * t8520 + 72.0_f64 * t13717 + 240.0_f64 * t7986 + 12.0_f64 * t10017 + 384.0_f64 * t7988 - 96.0_f64 * t7990 + 8.0_f64 * t13721 + t16345 + t16349 + t16350 + t16351 + 4.0_f64 * t12363 + t16353 - t16354;
    t50796
}
