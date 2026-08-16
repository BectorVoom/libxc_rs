//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1058/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1058(t1368: f64, t1464: f64, t285: f64, t2036: f64, t5887: f64, t281: f64, t4562: f64, t545: f64, t168: f64, t18344: f64, t286: f64, t475: f64, t5880: f64) -> (f64, f64, f64, f64, f64) {
    let t19107 = 0.81358876250083374227e-2_f64 * t1464 * t1368 * t285;
    let t19108 = t5887 * t2036;
    let t19117 = t281 * t4562 * t545 * t285;
    let t19121 = 0.91063310497738755577e0_f64 * t168 * t18344 * t286;
    let t19124 = t475 * t5880;
    (t19107, t19108, t19117, t19121, t19124)
}
