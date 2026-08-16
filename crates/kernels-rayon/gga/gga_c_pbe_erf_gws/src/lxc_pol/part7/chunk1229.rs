//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1229/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1229(t21387: f64, t21395: f64, t21412: f64, t21414: f64, t21424: f64, t21429: f64, t21445: f64, t21455: f64, t21462: f64, t21478: f64, t21494: f64, t21502: f64, t21513: f64, t21528: f64, t21534: f64, t21540: f64, t21544: f64, t21563: f64, t21565: f64, t21577: f64, t21580: f64, t21594: f64) -> (f64, f64) {
    let t21709 = -t21387 - t21395 + t21412 - t21414 - t21424 + t21429 + t21445 - t21455 + t21462 - t21478 - t21494;
    let t21711 = -t21502 + t21513 + t21528 - t21534 - t21540 + t21544 - t21563 - t21565 + t21577 + t21580 - t21594;
    (t21709, t21711)
}
