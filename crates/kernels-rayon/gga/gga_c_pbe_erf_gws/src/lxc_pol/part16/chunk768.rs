//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 768/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk768(t1924: f64, t723: f64, t1917: f64, t245: f64, t712: f64, t1903: f64, t708: f64, t703: f64, t713: f64, t1906: f64, t719: f64, t256: f64) -> (f64, f64, f64, f64, f64) {
    let t5433 = 2.0_f64 / 3.0_f64 * t1924 * t723;
    let t5434 = t245 * t1917;
    let t5436 = 0.2e-20_f64 * t712 * t5434;
    let t5437 = t708 * t1903;
    let t5441 = t703 * t713;
    let t5443 = 0.13506172839506172839e-1_f64 * t712 * t5441;
    let t5448 = t1906 * t719;
    let t5449 = t5448 * t256;
    (t5433, t5436, t5437, t5443, t5449)
}
