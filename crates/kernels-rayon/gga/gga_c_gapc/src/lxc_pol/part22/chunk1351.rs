//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1351/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1351(t35404: f64, t35406: f64, t35409: f64, t35412: f64, t35397: f64, t36332: f64, t36333: f64, t36334: f64, t36335: f64, t36336: f64, t36337: f64, t36338: f64, t36340: f64) -> f64 {
    let t36341 = 0.17379648562707520765e-3_f64 * t35404;
    let t36342 = 0.14024275817241799902e-4_f64 * t35406;
    let t36343 = 0.2530696388073708253e-5_f64 * t35409;
    let t36344 = 0.14762395597096631476e-5_f64 * t35412;
    let t36345 = t36332 - t36333 - t36334 - t36335 - t36336 - t36337 - t36338 + 0.53949325746737929042e-3_f64 * t35397 - t36340 - t36341 + t36342 - t36343 - t36344;
    t36345
}
