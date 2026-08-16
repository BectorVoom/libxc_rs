//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1392/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1392(t26135: f64, t7423: f64, t24969: f64, t7467: f64, t112: f64, t33761: f64, t116362: f64, t120786: f64, t120788: f64, t120789: f64, t1458: f64, t31284: f64, t31937: f64, t33195: f64, t4072: f64, t671: f64, t8508: f64) -> f64 {
    let t123272 = t7423 * t26135;
    let t123274 = t24969 * t7467;
    let t123277 = t33761 * t112;
    let t123280 = t31284 + 0.135e2_f64 * t31937 * t4072 + t8508 + 0.135e2_f64 * t116362 * t1458 + t120786 + 0.135e2_f64 * t123272 + 0.135e2_f64 * t123274 + t120788 + 27.0_f64 * t120789 + 0.135e2_f64 * t123277 * t671 + t33195;
    t123280
}
