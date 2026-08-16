//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 573/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk573(t2354: f64, t27469: f64, t446: f64, t1882: f64, t6896: f64, t18: f64, t6135: f64, t3281: f64, t10157: f64, t6852: f64, t713: f64, t24526: f64, t992: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27470 = t2354 * t27469;
    let t27471 = t446 * t27470;
    let t27473 = t1882 * t6896;
    let t27475 = t6135 * t18;
    let t27476 = t2354 * t27475;
    let t27477 = t3281 * t27476;
    let t27480 = t10157 * t6852 * t713;
    let t27481 = t446 * t27480;
    let t27483 = t24526 * t992;
    (t27471, t27473, t27475, t27477, t27481, t27483)
}
