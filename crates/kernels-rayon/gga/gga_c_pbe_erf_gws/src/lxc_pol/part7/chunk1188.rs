//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1188/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1188(t21105: f64, t339: f64, t4867: f64, t850: f64, t851: f64, t860: f64, t6588: f64, t899: f64, t900: f64, t907: f64, t6593: f64, t855: f64, t859: f64, param_a_c: f64) -> (f64, f64, f64, f64) {
    let t21106 = param_a_c * t21105;
    let t21111 = t4867 * t339;
    let t21115 = t850 * t851 * t21111 * t860 / 96.0_f64;
    let t21117 = t899 * t900 * t6588;
    let t21118 = t21117 * t907;
    let t21121 = t855 * t6593 * t859;
    (t21106, t21115, t21118, t21121)
}
