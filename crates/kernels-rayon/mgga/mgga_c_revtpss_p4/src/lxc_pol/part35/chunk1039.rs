//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1039/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1039(t2411: f64, t33: f64, t112: f64, t239: f64, t624: f64, t655: f64, t2339: f64, t68: f64, t555: f64, t7063: f64) -> (f64, f64, f64, f64, f64) {
    let t25759 = t2411 * t33;
    let t25821 = t239 * t112;
    let t25823 = t624 * t655;
    let t25826 = t68 * t2339;
    let t25875 = t7063 * t555;
    (t25759, t25821, t25823, t25826, t25875)
}
