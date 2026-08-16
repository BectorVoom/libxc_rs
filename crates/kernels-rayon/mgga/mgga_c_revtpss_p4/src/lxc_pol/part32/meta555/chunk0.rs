//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1874/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1874(t5665: f64, t94497: f64, t14036: f64, t25997: f64, t13941: f64, t94423: f64, t14005: f64, t5706: f64, t94429: f64, t1941: f64, t9817: f64, t5651: f64, t7028: f64, t9736: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98174 = t94497 * t5665;
    let t98180 = t25997 * t14036;
    let t98185 = t94423 * t13941;
    let t98187 = t94423 * t14005;
    let t98193 = t94429 * t5706;
    let t98196 = t1941 * t9817;
    let t98200 = t9736 * t7028 * t5651;
    (t98174, t98180, t98185, t98187, t98193, t98196, t98200)
}
