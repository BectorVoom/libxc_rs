//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1227/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1227(t11598: f64, t8765: f64, t11387: f64, t19916: f64, t5553: f64, t1030: f64, t26034: f64, t34077: f64, t20501: f64, t33411: f64, t19511: f64, t33415: f64) -> (f64, f64, f64, f64, f64) {
    let t35143 = t11598 * t8765;
    let t35146 = t5553 * t11387 * t19916;
    let t35149 = t1030 * t34077 * t26034;
    let t35152 = t1030 * t33411 * t20501;
    let t35155 = t1030 * t33415 * t19511;
    (t35143, t35146, t35149, t35152, t35155)
}
