//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 227/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk227(t240: f64, t853: f64, t72: f64, t213: f64, t251: f64, t256: f64) -> (f64, f64, f64, f64, f64) {
    let t854 = t240 * t853;
    let t855 = t854 * t72;
    let t865 = t213 * t251;
    let t866 = t256 * t256;
    let t867 = 1.0_f64 / t866;
    (t854, t855, t865, t866, t867)
}
