//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1795/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1795(t231: f64, t836: f64, t886: f64, t25392: f64, t1950: f64, t867: f64, t786: f64) -> (f64, f64, f64, f64) {
    let t25394 = t886 * t836 * t231;
    let t25395 = t25392 * t25394;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    (t25394, t25395, t25398, t25399)
}
