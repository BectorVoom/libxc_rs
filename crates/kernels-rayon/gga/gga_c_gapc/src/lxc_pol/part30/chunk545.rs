//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 545/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk545(t3139: f64, t3144: f64, t1013: f64, t608: f64, t1016: f64, t1019: f64, t561: f64, t1457: f64, t190: f64, t1453: f64, t134: f64, t200: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3145 = t3139 * t3144;
    let t3147 = t1013 * t608;
    let t3150 = t561 * t1016 * t1019;
    let t3152 = t1457 * t190;
    let t3153 = t3152 * t1453;
    let t3155 = t134 * t200;
    (t3145, t3147, t3150, t3152, t3153, t3155)
}
