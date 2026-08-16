//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 838/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk838(t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64) -> (f64, f64) {
    let t3579 = 0.19755555555555555556e-1_f64 * t3356;
    let t3584 = t3579 - 0.9877777777777777778e-2_f64 * t3358 - 0.9877777777777777778e-2_f64 * t3365 + 0.29633333333333333334e-1_f64 * t3370 + 0.14816666666666666667e-1_f64 * t3374;
    (t3579, t3584)
}
