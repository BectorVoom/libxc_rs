//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 938/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk938(t11642: f64, t11701: f64, t11751: f64, t11799: f64, t11850: f64, t11919: f64, t11976: f64, t12029: f64, t225: f64, t385: f64, t3270: f64, t999: f64) -> (f64, f64, f64) {
    let t12032 = t11642 + t11701 + t11751 + t11799 + t11850 + t11919 + t11976 + t12029;
    let t12034 = t12032 * t225 * t385;
    let t12039 = t999 * t3270;
    (t12032, t12034, t12039)
}
