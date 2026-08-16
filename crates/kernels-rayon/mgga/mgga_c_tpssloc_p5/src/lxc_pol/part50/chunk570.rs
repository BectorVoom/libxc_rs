//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 570/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk570(t4370: f64, t894: f64, t1547: f64, t2815: f64, t896: f64, t901: f64, t1553: f64, t699: f64, t2826: f64, t4338: f64, t136: f64, t4343: f64, t908: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4371 = t894 * t4370;
    let t4378 = t2815 * t1547;
    let t4379 = t4378 * t896;
    let t4381 = t901 * t4370;
    let t4384 = t699 * t1553;
    let t4386 = t2826 * t4338;
    let t4387 = t136 * t4386;
    let t4389 = t908 * t4343;
    (t4371, t4379, t4381, t4384, t4387, t4389)
}
