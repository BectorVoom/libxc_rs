//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 787/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk787(t4579: f64, t550: f64, t553: f64, t1339: f64, t1971: f64, t4585: f64, t2704: f64, t2718: f64, t7: f64, t226: f64, t1989: f64, t679: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t5895 = 0.59261670986728442646e-2_f64 * t550 * t4579 * t553;
    let t5898 = 0.14862827083471493416e-2_f64 * t1339 * t4585 * t1971;
    let t5902 = 0.12833333333333333333e1_f64 * t2704 - 20.0_f64 / 27.0_f64 * t2718;
    let t5903 = t5902 * pi;
    let t5904 = t5903 * t7;
    let t5906 = 4.0_f64 / 3.0_f64 * t226 * t5904;
    let t5910 = t1989 * t679;
    (t5895, t5898, t5906, t5910)
}
