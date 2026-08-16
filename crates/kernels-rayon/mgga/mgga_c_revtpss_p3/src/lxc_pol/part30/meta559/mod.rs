//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2001;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta559(t228: f64, t25273: f64, t802: f64, t25277: f64, t2707: f64, t25282: f64, t9802: f64, t243: f64, t7021: f64, t2732: f64, t64: f64, t9731: f64, t2710: f64, t826: f64, t10631: f64, t10886: f64, t7028: f64, t159: f64, t8779: f64, t218: f64, t816: f64, t10685: f64, t1946: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92968, t92969, t92971, t92976, t92979, t92986) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2001(t228, t25273, t802, t25277, t2707, t25282, t9802, t243, t7021, t2732, t64, t9731);
        let (t92989, t92991, t92993, t92996, t92997) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2002(t2710, t826, t92986, t10631, t10886, t7028, t159, t8779, t218, t816, t10685, t1946);
    (t92968, t92969, t92971, t92976, t92979, t92986, t92989, t92991, t92993, t92996, t92997)
}
