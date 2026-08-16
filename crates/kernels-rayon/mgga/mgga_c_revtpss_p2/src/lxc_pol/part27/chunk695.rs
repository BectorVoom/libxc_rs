//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 695/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk695(t218: f64, t7021: f64, t816: f64, t1941: f64, t228: f64, t802: f64, t240: f64, t64: f64) -> (f64, f64, f64, f64) {
    let t7023 = t7021 * t218 * t816;
    let t7024 = 7.0_f64 / 288.0_f64 * t7023;
    let t7025 = t1941 * t228;
    let t7026 = t7025 * t802;
    let t7028 = t64 * t240;
    (t7024, t7025, t7026, t7028)
}
