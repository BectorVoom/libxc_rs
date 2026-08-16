//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 984/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk984(t1091: f64, t140490: f64, t140495: f64, t140508: f64, t140513: f64, t149674: f64, t2354: f64, t2404: f64, t24204: f64, t28012: f64, t28015: f64, t28027: f64, t28032: f64, t28038: f64, t28042: f64, t3051: f64, t33496: f64, t33499: f64, t33537: f64, t35259: f64, t6002: f64, t683: f64, t7436: f64, t7441: f64) -> f64 {
    let t149700 = t33499 * t28027 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t6002 * t683 * t7441 * t28032 + 2.0_f64 / 27.0_f64 * t6002 * t2404 * t7441 * t28038 + t149674 / 54.0_f64 - t28015 * t33537 / 18.0_f64 - t6002 * t140513 * t28042 / 3.0_f64 + t28015 * t33496 / 9.0_f64 - t24204 * t35259 / 9.0_f64 - t6002 * t2354 * t140490 * t1091 / 9.0_f64 - t6002 * t2354 * t140508 * t1091 / 9.0_f64 + t7436 * t3051 * t28012 / 9.0_f64 - t6002 * t2354 * t140495 * t1091 / 18.0_f64;
    t149700
}
