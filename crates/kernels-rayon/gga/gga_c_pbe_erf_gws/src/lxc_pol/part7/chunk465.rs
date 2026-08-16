//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 465/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk465(t43: f64, t50: f64, t2059: f64, t2062: f64, t1524: f64, t1526: f64, t1529: f64, t1531: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t2063 = t2059 * t2062;
    let t2064 = 0.63272429661648472106e0_f64 * t2063;
    let t2068 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t1524 + 2.0_f64 / 3.0_f64 * t1526);
    let t2072 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t1529 + 2.0_f64 / 3.0_f64 * t1531);
    let t2074 = t2068 / 2.0_f64 + t2072 / 2.0_f64;
    (t2064, t2074)
}
