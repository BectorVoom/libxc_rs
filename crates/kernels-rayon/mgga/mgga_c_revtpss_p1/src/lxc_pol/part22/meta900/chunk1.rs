//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3094/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3094(t366: f64, t53865: f64, t1025: f64, t371: f64, t4852: f64, t676: f64, t225: f64, t53014: f64, t11656: f64, t15734: f64, t11670: f64, t370: f64) -> (f64, f64, f64, f64, f64) {
    let t53866 = t53865 * t366;
    let t53875 = t1025 * t371 * t676 * t4852;
    let t53877 = t53014 * t225;
    let t53881 = t11656 * t15734;
    let t53884 = t11670 * t370;
    (t53866, t53875, t53877, t53881, t53884)
}
