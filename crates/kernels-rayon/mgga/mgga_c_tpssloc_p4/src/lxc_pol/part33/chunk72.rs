//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 72/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk72(t40: f64, t52: f64, t73: f64, t194: f64, t76: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t195 = t73 * t73;
    let t196 = piecewise3(t146, t194, t195);
    let t197 = t76 * t76;
    let t198 = piecewise3(t150, t194, t197);
    let t200 = t196 / 2.0_f64 + t198 / 2.0_f64;
    (t195, t197, t200)
}
