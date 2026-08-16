//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 630/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk630(t168: f64, t270: f64, t5589: f64, t153: f64, t274: f64, t4573: f64, t147: f64, t285: f64, t4576: f64, t281: f64, t131: f64, t2029: f64) -> (f64, f64, f64, f64, f64) {
    let t5592 = 0.19455129084526283664e0_f64 * t168 * t5589 * t270;
    let t5595 = 0.4429070076315393047e1_f64 * t153 * t4573 * t274;
    let t5615 = t147 * t4576 * t285;
    let t5617 = 0.11974234010254609094e-1_f64 * t281 * t5615;
    let t5621 = 1.0_f64 / t2029 / t131;
    (t5592, t5595, t5615, t5617, t5621)
}
