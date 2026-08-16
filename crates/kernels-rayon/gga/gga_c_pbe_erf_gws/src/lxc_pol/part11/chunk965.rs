//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 965/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk965(t1114: f64, t19817: f64, t19905: f64, t19839: f64, t833: f64, t1146: f64, t6729: f64, t1125: f64, t21121: f64, t20189: f64, t3133: f64, t20693: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26755 = t1114 * t19817;
    let t26958 = t1114 * t19905;
    let t27077 = t1114 * t19839 * t833;
    let t27079 = t6729 * t1146;
    let t27197 = t1125 * t21121;
    let t27222 = t20189 * t3133;
    let t27556 = t1114 * t20693;
    (t26755, t26958, t27077, t27079, t27197, t27222, t27556)
}
