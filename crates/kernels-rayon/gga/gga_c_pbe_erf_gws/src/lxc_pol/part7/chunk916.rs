//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 916/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk916(t1651: f64, t5287: f64, t587: f64, t5018: f64, t5394: f64, t1620: f64, t1809: f64, t5033: f64, t617: f64, t1815: f64, t639: f64, t661: f64) -> (f64, f64, f64, f64) {
    let t17207 = t587 * t1651 * t5287;
    let t17208 = 32.0_f64 / 135.0_f64 * t17207;
    let t17210 = t587 * t5018 * t5394;
    let t17211 = 32.0_f64 / 15.0_f64 * t17210;
    let t17215 = 64.0_f64 / 15.0_f64 * t1620 * t1809 * t5033 * t617;
    let t17219 = 32.0_f64 / 15.0_f64 * t639 * t1815 * t5033 * t661;
    (t17208, t17211, t17215, t17219)
}
