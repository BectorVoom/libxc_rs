//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 511/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk511(t30: f64, t33: f64, t1312: f64, t1502: f64, t1518: f64, t1468: f64, t513: f64, t1711: f64, t516: f64, t162: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1847 = 2.0_f64 * t1312 * t1518 + t1502;
    let t1851 = piecewise3(t31, 0.0_f64, 4.0_f64 / 3.0_f64 * t513 * t1468);
    let t1854 = piecewise3(t34, 0.0_f64, 4.0_f64 / 3.0_f64 * t516 * t1711);
    let t1856 = (t1851 + t1854) * t162;
    (t1847, t1856)
}
