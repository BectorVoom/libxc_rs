//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 461/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk461(t573: f64, t610: f64, t1827: f64, t587: f64, t108: f64, t1403: f64, t1407: f64, t1413: f64, t1416: f64, t726: f64, t728: f64, t92: f64, t93: f64) -> (f64, f64, f64, f64) {
    let t1828 = t573 * t610;
    let t1829 = t1827 * t1828;
    let t1831 = 8.0_f64 / 45.0_f64 * t587 * t1829;
    let t1841 = (20.0_f64 / 9.0_f64 * t92 * t1403 + 4.0_f64 / 3.0_f64 * t726 * t1407 + 20.0_f64 / 9.0_f64 * t93 * t1413 + 4.0_f64 / 3.0_f64 * t728 * t1416) * t108;
    (t1828, t1829, t1831, t1841)
}
