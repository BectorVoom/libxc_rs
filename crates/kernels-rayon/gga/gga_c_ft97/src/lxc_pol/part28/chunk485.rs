//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 485/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk485(t7312: f64, t7369: f64, t7239: f64, t7366: f64, t2112: f64, t1369: f64, t28: f64, t586: f64, t7339: f64, t1985: f64, t27: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7370 = t7369 * t7312;
    let t7372 = t7366 * t7239 * t7370;
    let t7374 = t2112 * t7312;
    let t7376 = t1369 * t28 * t7374;
    let t7378 = t586 * t7339;
    let t7380 = t1369 * t28 * t7378;
    let t7382 = t1985 * t7312;
    let t7384 = t89 * t27 * t7382;
    (t7370, t7372, t7374, t7376, t7378, t7380, t7382, t7384)
}
