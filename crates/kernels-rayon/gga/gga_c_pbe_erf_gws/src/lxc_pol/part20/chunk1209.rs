//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1209/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1209(t11661: f64, t4395: f64, t11583: f64, t810: f64, t3703: f64, t2079: f64, t3780: f64, t3306: f64, t8589: f64, t2395: f64, t19894: f64, t3912: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38537 = t4395 * t11661;
    let t38545 = t11583 * t810;
    let t39052 = t3703 * param_a_c;
    let t39061 = t2079 * t3780;
    let t39460 = t8589 * t3306;
    let t39579 = t2395 * t3703;
    let t39689 = t3912 * t19894;
    (t38537, t38545, t39052, t39061, t39460, t39579, t39689)
}
