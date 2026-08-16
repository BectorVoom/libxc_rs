//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1075/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1075(t3425: f64, t3454: f64, t5548: f64, t587: f64, t1017: f64, t12472: f64, t1827: f64, t12717: f64, t2615: f64, t12767: f64, t7527: f64, t1620: f64, t1809: f64, t41459: f64, t954: f64) -> (f64, f64, f64, f64, f64) {
    let t47319 = 32.0_f64 / 15.0_f64 * t587 * t5548 * t3425 * t3454;
    let t47323 = 32.0_f64 / 15.0_f64 * t587 * t1827 * t12472 * t1017;
    let t47325 = 16.0_f64 / 15.0_f64 * t2615 * t12717;
    let t47327 = 32.0_f64 / 15.0_f64 * t7527 * t12767;
    let t47331 = 32.0_f64 / 45.0_f64 * t1620 * t1809 * t41459 * t954;
    (t47319, t47323, t47325, t47327, t47331)
}
