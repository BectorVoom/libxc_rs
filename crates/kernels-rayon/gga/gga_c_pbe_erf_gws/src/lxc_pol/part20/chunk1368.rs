//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1368/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1368(t14138: f64, t57508: f64, t2306: f64, t331: f64, t3780: f64, t3074: f64, t833: f64, t15177: f64, t3979: f64, t15275: f64, t840: f64, t15213: f64, t4414: f64) -> (f64, f64, f64, f64, f64) {
    let t57509 = t57508 * t14138;
    let t57512 = t2306 * t3780 * t331;
    let t57514 = t3074 * t57512 * t833;
    let t57516 = t3979 * t15177;
    let t57518 = t840 * t15275;
    let t57534 = t4414 * t15213;
    (t57509, t57514, t57516, t57518, t57534)
}
