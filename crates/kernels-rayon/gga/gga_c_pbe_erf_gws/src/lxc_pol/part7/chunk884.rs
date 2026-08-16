//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 884/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk884(t198: f64, t7776: f64, t185: f64, t1893: f64, t5470: f64, t1627: f64, t5005: f64, t1624: f64, t16649: f64, t1820: f64, t5018: f64, t5308: f64) -> (f64, f64, f64, f64, f64) {
    let t16843 = t7776 * t198;
    let t16845 = 112.0_f64 / 1215.0_f64 * t185 * t16843;
    let t16847 = 16.0_f64 / 15.0_f64 * t5470 * t1893;
    let t16849 = 32.0_f64 / 9.0_f64 * t1627 * t5005;
    let t16851 = 16.0_f64 / 5.0_f64 * t16649 * t1624;
    let t16853 = t1820 * t5018 * t5308;
    (t16845, t16847, t16849, t16851, t16853)
}
