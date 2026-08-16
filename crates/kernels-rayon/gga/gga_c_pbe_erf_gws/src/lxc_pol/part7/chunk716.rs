//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 716/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk716(t101: f64, t5880: f64, t1590: f64, t524: f64, t142: f64, t1378: f64, t1971: f64, t5701: f64, t4579: f64, t550: f64, t553: f64, t1339: f64, t4585: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5881 = t101 * t5880;
    let t5887 = t524 * t1590;
    let t5888 = t5887 * t142;
    let t5891 = t5701 * t1378 * t1971;
    let t5895 = 0.59261670986728442646e-2_f64 * t550 * t4579 * t553;
    let t5898 = 0.14862827083471493416e-2_f64 * t1339 * t4585 * t1971;
    (t5881, t5887, t5888, t5891, t5895, t5898)
}
