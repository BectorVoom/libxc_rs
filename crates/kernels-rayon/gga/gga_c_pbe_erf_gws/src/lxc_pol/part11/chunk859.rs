//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 859/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk859(t13496: f64, t2168: f64, t13220: f64, t6384: f64, t904: f64, t11946: f64, t11600: f64, t3180: f64, t13086: f64, t933: f64, t13125: f64, t6472: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13498 = t2168 * t13496 / 16.0_f64;
    let t13500 = t6384 * t904 * t13220;
    let t13503 = 7.0_f64 / 24.0_f64 * t11946;
    let t13505 = t11600 * t3180 / 16.0_f64;
    let t13507 = t933 * t904 * t13086;
    let t13510 = t13125 * t6472;
    (t13498, t13500, t13503, t13505, t13507, t13510)
}
