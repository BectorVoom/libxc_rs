//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 949/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk949(t11499: f64, t185: f64, t11496: f64, t3116: f64, t436: f64, t3115: f64, t11388: f64, t3123: f64, t1453: f64, t474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11500 = t185 * t11499;
    let t11501 = t11500 * t11496;
    let t11503 = t436 * t3116;
    let t11504 = t3115 * t11503;
    let t11506 = t11388 * t3123;
    let t11508 = t474 * t1453;
    (t11500, t11501, t11503, t11504, t11506, t11508)
}
