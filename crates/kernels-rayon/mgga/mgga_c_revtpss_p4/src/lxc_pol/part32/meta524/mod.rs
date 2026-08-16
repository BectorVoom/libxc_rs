//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1828;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1829;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta524(t802: f64, t92968: f64, t25282: f64, t9802: f64, t243: f64, t7021: f64, t64: f64, t9731: f64, t2710: f64, t826: f64, t10631: f64, t10886: f64, t7028: f64, t159: f64, t8779: f64, t218: f64, t816: f64, t10685: f64, t1946: f64, t10671: f64, t7033: f64, t25255: f64, t2689: f64, t10690: f64, t1945: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92969, t92975, t92978, t92986, t92988, t92991) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1828(t802, t92968, t25282, t9802, t243, t7021, t64, t9731, t2710, t826, t10631, t10886, t7028);
        let (t92993, t92995, t92997, t92999, t93001, t93007) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1829(t159, t8779, t218, t816, t10685, t1946, t10671, t7033, t25255, t2689, t10690, t1945, t9646);
    (t92969, t92975, t92978, t92986, t92988, t92991, t92993, t92995, t92997, t92999, t93001, t93007)
}
