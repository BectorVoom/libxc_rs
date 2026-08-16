//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1161/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1161(t11808: f64, t11983: f64, t11772: f64, t29692: f64, t11795: f64, t9387: f64, t11508: f64, t3402: f64, t7944: f64, t11513: f64, t7259: f64, t11822: f64, t7511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33563 = t11808 * t11983;
    let t33565 = t11772 * t29692;
    let t33567 = t9387 * t11795;
    let t33570 = t3402 * t11508 * t7944;
    let t33576 = t7259 * t11513 * t7944;
    let t33578 = t11822 * t7511;
    (t33563, t33565, t33567, t33570, t33576, t33578)
}
