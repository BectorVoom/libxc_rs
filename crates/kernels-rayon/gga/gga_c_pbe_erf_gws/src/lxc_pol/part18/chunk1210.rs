//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1210/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1210(t8546: f64, t944: f64, t2416: f64, t3906: f64, t13252: f64, t36888: f64, t274: f64, t3111: f64, t9607: f64, t1123: f64, t745: f64, t3222: f64, t39052: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43260 = t8546 * t944;
    let t43526 = t3906 * t2416;
    let t44196 = t36888 * t13252;
    let t44200 = t3111 * t274;
    let t44201 = t9607 * t44200;
    let t44205 = t1123 * t745;
    let t44206 = t9607 * t44205;
    let t45096 = t39052 * t3222;
    (t43260, t43526, t44196, t44201, t44206, t45096)
}
