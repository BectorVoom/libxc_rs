//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 542/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk542(t138: f64, t1577: f64, t2902: f64, t3671: f64, t3675: f64, t3683: f64, t514: f64, t985: f64, t101: f64) -> (f64, f64) {
    let t3685 = t138 * t3671 + 2.0_f64 * t1577 * t3675 - 2.0_f64 * t2902 * t985 - t3683 * t514;
    let t3686 = t101 * t3685;
    (t3685, t3686)
}
