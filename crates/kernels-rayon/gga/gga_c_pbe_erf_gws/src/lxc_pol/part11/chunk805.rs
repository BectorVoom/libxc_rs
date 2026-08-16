//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 805/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk805(t12534: f64, t12536: f64, t12540: f64, t12542: f64, t12546: f64, t12548: f64, t12552: f64, t12554: f64, t12558: f64, t12562: f64, t12566: f64, t12568: f64, t12569: f64, t12570: f64, t12574: f64, t12578: f64, t12582: f64, t12587: f64) -> f64 {
    let t13006 = t12534 - t12536 - t12540 + t12542 - t12546 + t12548 + t12552 + t12554 + t12558 + t12562 - t12566 + t12568 + t12569 + t12570 + t12574 + t12578 - t12582 - t12587;
    t13006
}
