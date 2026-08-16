//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 500/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk500(t1642: f64, t981: f64, t1594: f64, t986: f64, t341: f64) -> (f64, f64, f64) {
    let t1644 = 0.5848223622634646207e0_f64 * t981 * t1642;
    let t1646 = -t986 - 0.83333333333333333333e-2_f64 * t1594;
    let t1647 = t1646 * t341;
    (t1644, t1646, t1647)
}
