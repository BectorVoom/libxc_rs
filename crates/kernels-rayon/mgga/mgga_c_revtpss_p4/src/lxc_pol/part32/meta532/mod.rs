//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1837;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta532(t3981: f64, t94443: f64, t25981: f64, t820: f64, t843: f64, t2681: f64, t7262: f64, t1401: f64, t533: f64, t816: f64, t92993: f64, t7259: f64, t9709: f64, t1389: f64, t3964: f64, t92986: f64, t7028: f64, t9736: f64, t9737: f64, t26009: f64, t9802: f64, t64: f64, t9990: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94444, t94455, t94459, t94460, t94471, t94473) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1837(t3981, t94443, t25981, t820, t843, t2681, t7262, t1401, t533, t816, t92993, t7259, t9709);
        let (t94476, t94479, t94483, t94491, t94497) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1838(t1389, t3964, t92986, t7028, t9736, t9737, t26009, t9802, t64, t9990, t2482, t596, t7262);
    (t94444, t94455, t94459, t94460, t94471, t94473, t94476, t94479, t94483, t94491, t94497)
}
