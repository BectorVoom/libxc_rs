//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 796/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk796(t33: f64, t516: f64, t1113: f64, t3881: f64, t1348: f64, t3351: f64, t9351: f64, t9357: f64, t9614: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t9615 = t33 * t33;
    let t9617 = 1.0_f64 / t516 / t9615;
    let t9620 = t3881 * t1113;
    let t9626 = piecewise3(t34, 0.0_f64, 8.0_f64 / 27.0_f64 * t9617 * t9351 - 2.0_f64 / 3.0_f64 * t9620 * t3351 + 2.0_f64 / 3.0_f64 * t1348 * t9357);
    let t9628 = t9614 / 2.0_f64 + t9626 / 2.0_f64;
    t9628
}
