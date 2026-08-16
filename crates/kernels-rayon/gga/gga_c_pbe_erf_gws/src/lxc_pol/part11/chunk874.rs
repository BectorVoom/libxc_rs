//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 874/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk874(t13505: f64, t13514: f64, t13520: f64, t13522: f64, t13527: f64, t13529: f64, t13538: f64, t13567: f64, t13569: f64, t13575: f64, t13582: f64, t13583: f64, t13602: f64, t6597: f64) -> f64 {
    let t13675 = -t6597 - t13505 - t13514 + t13520 - t13522 + t13527 + t13529 + t13538 - t13567 - t13569 - t13575 + t13582 + t13583 - t13602;
    t13675
}
