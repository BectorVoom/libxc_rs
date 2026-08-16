//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1120/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1120(t64: f64, t9731: f64, t2710: f64, t826: f64, t10631: f64, t10886: f64, t7028: f64, t159: f64, t8779: f64, t218: f64, t816: f64, t10685: f64, t1946: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92986 = t64 * t9731;
    let t92988 = t2710 * t92986 * t826;
    let t92991 = t10886 * t7028 * t10631;
    let t92993 = t8779 * t159;
    let t92995 = t92993 * t218 * t816;
    let t92997 = t1946 * t10685;
    (t92986, t92988, t92991, t92993, t92995, t92997)
}
