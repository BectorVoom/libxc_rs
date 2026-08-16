//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 931/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk931(t8197: f64, t542: f64, t974: f64, t496: f64, t127: f64, t1504: f64, t5810: f64, t5819: f64, t5836: f64, t8181: f64, t8182: f64, t8186: f64, t8187: f64, t8193: f64, t8194: f64) -> (f64, f64, f64) {
    let t8198 = 0.64956111111111111111e0_f64 * t8197;
    let t8199 = t542 * t974;
    let t8200 = t496 * t8199;
    let t8202 = -0.195872e1_f64 * t5810 - t8181 - t8182 - t5819 / 2.0_f64 - t8186 - 0.293808e2_f64 * t127 * t8187 * t1504 - t8193 - 0.146904e1_f64 * t127 * t8194 - t8198 - 2.0_f64 / 9.0_f64 * t8200 + t5836;
    (t8198, t8199, t8202)
}
