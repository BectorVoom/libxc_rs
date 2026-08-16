//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1265/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1265(t36327: f64, t36333: f64, t37960: f64, t40490: f64, t40493: f64, t40497: f64, t40501: f64, t40505: f64, t40507: f64, t40511: f64, t40515: f64, t40517: f64, t40519: f64, t40521: f64, t40523: f64, t40525: f64, t40527: f64, t40529: f64) -> f64 {
    let t42150 = 0.31448092289604152069e-3_f64 * t40490 + 0.25724410870841842183e-2_f64 * t40493 + 0.42874018118069736972e-3_f64 * t40497 + 0.21437009059034868486e-2_f64 * t40501 + 0.12862205435420921092e-2_f64 * t40505 - 0.24009450146119052705e-1_f64 * t40507 - 0.37737710747524982484e-2_f64 * t40511 + 0.12579236915841660828e-2_f64 * t40515 - 0.13719685797782315831e-1_f64 * t40517 - 0.13719685797782315831e-1_f64 * t40519 - 0.68598428988911579156e-2_f64 * t40521 + 0.68598428988911579156e-2_f64 * t40523 - 0.37737710747524982482e-1_f64 * t36327 - 0.17149607247227894789e-2_f64 * t40525 - t37960 - 0.17149607247227894789e-2_f64 * t40527 + 0.25724410870841842184e-1_f64 * t36333 - 0.68598428988911579156e-2_f64 * t40529;
    t42150
}
