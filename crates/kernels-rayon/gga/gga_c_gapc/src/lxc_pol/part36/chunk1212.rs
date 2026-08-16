//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1212/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1212(t1030: f64, t26034: f64, t34077: f64, t20501: f64, t33411: f64, t19511: f64, t33415: f64, t11388: f64, t3065: f64, t11479: f64, t1912: f64, t5285: f64) -> (f64, f64, f64, f64, f64) {
    let t35149 = t1030 * t34077 * t26034;
    let t35152 = t1030 * t33411 * t20501;
    let t35155 = t1030 * t33415 * t19511;
    let t35157 = t11388 * t3065;
    let t35160 = t5285 * t11479 * t1912;
    (t35149, t35152, t35155, t35157, t35160)
}
