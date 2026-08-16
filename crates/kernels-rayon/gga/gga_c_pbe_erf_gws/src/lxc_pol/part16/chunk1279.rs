//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1279/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1279(t14028: f64, t3299: f64, t14567: f64, t6608: f64, t9484: f64, t14535: f64, t2115: f64, t14538: f64, t51282: f64, t2129: f64, t51306: f64, t9500: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54198 = t14028 * t3299;
    let t54201 = t6608 * t9484 * t14567;
    let t54203 = t2115 * t14535;
    let t54205 = t14538 * t51282;
    let t54207 = t2129 * t14535;
    let t54209 = t51306 * t9500;
    (t54198, t54201, t54203, t54205, t54207, t54209)
}
