//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1143/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1143(t1620: f64, t1809: f64, t3473: f64, t3562: f64, t1044: f64, t12513: f64, t1815: f64, t639: f64, t16801: f64, t42094: f64, t954: f64, t1821: f64, t47438: f64, t587: f64) -> (f64, f64, f64, f64) {
    let t48187 = 16.0_f64 / 15.0_f64 * t1620 * t1809 * t3473 * t3562;
    let t48191 = 16.0_f64 / 45.0_f64 * t639 * t1815 * t12513 * t1044;
    let t48195 = 32.0_f64 / 15.0_f64 * t639 * t16801 * t42094 * t954;
    let t48198 = 32.0_f64 / 45.0_f64 * t587 * t1821 * t47438;
    (t48187, t48191, t48195, t48198)
}
