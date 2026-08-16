//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1289/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1289(t2752: f64, t31429: f64, t193: f64, t201: f64, t8565: f64, t10143: f64, t111: f64, t31699: f64, t31650: f64, t6883: f64, t31608: f64, t1377: f64, t7213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114992 = t31429 * t2752;
    let t115009 = t193 * t201 * t8565;
    let t115027 = t8565 * t10143;
    let t115241 = t31699 * t111;
    let t115292 = t6883 * t31650;
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    (t114992, t115009, t115027, t115241, t115292, t115294, t115296)
}
