//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 760/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk760(t1037: f64, t10629: f64, t3513: f64, t7527: f64, t1044: f64, t10691: f64, t1621: f64, t1620: f64, t2607: f64, t3553: f64, t11032: f64, t2612: f64, t3519: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12436 = 8.0_f64 / 15.0_f64 * t10629 * t1037;
    let t12438 = 8.0_f64 / 5.0_f64 * t7527 * t3513;
    let t12439 = t10691 * t1044;
    let t12440 = t1621 * t12439;
    let t12442 = 4.0_f64 / 5.0_f64 * t1620 * t12440;
    let t12443 = t2607 * t3553;
    let t12444 = t1621 * t12443;
    let t12446 = 4.0_f64 / 5.0_f64 * t1620 * t12444;
    let t12448 = 4.0_f64 / 15.0_f64 * t11032 * t1037;
    let t12450 = 4.0_f64 / 15.0_f64 * t2612 * t3519;
    (t12436, t12438, t12439, t12440, t12442, t12443, t12444, t12446, t12448, t12450)
}
