//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 764/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk764(t12501: f64, t1691: f64, t11: f64, t12350: f64, t1642: f64, t625: f64, t2672: f64, t3354: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12502 = t1691 * t12501;
    let t12503 = t11 * t12502;
    let t12505 = t1642 * t12350;
    let t12506 = t625 * t12505;
    let t12507 = t11 * t12506;
    let t12509 = t2672 * t3354;
    (t12502, t12503, t12505, t12506, t12507, t12509)
}
