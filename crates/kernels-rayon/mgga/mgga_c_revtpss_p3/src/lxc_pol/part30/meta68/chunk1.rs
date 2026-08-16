//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 443/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk443(t30: f64, t33: f64, t1320: f64, t521: f64, t513: f64, t605: f64, t1113: f64, t516: f64, t162: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1322 = 4.0_f64 * t1320 * t521;
    let t1325 = piecewise3(t31, 0.0_f64, 4.0_f64 / 3.0_f64 * t513 * t605);
    let t1328 = piecewise3(t34, 0.0_f64, 4.0_f64 / 3.0_f64 * t516 * t1113);
    let t1330 = (t1325 + t1328) * t162;
    (t1322, t1330)
}
