//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 348/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk348(t43: f64, t50: f64, t1098: f64, t312: f64, t964: f64, t965: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1099 = t1098 * t312;
    let t1101 = piecewise3(t44, 0.0_f64, 2.0_f64 / 3.0_f64 * t964);
    let t1103 = piecewise3(t51, 0.0_f64, 2.0_f64 / 3.0_f64 * t965);
    let t1105 = t1101 / 2.0_f64 + t1103 / 2.0_f64;
    (t1099, t1105)
}
