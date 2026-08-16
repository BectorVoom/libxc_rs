//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1220/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1220(t25851: f64, t4254: f64, t1310: f64, t25832: f64, t651: f64, t116: f64, t25168: f64, t1962: f64, t41154: f64, t11061: f64, t30: f64, t27383: f64, t50066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92733 = 6.0_f64 * t4254 * t25851;
    let t92736 = 6.0_f64 * t651 * t1310 * t25832;
    let t92737 = t25168 * t116;
    let t92742 = t1962 * t41154;
    let t92743 = t30 * t11061;
    let t92747 = t27383 * t50066;
    (t92733, t92736, t92737, t92742, t92743, t92747)
}
