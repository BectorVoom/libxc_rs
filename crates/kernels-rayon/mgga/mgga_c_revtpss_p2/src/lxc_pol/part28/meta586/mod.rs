//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2052;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta586(t3981: f64, t94443: f64, t25986: f64, t2661: f64, t9930: f64, t25981: f64, t820: f64, t843: f64, t4006: f64, t2681: f64, t7262: f64, t1401: f64, t25997: f64, t9905: f64, t533: f64, t816: f64, t92993: f64, t7259: f64, t9709: f64, t1389: f64, t3964: f64, t92986: f64, t7028: f64, t9736: f64, t9737: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94444, t94449, t94456, t94459, t94460) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2052(t3981, t94443, t25986, t2661, t9930, t25981, t820, t843, t4006, t2681, t7262, t1401);
        let (t94468, t94472, t94474, t94477, t94479) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2053(t25997, t9905, t533, t816, t92993, t7259, t9709, t1389, t3964, t92986, t7028, t9736, t9737);
    (t94444, t94449, t94456, t94459, t94460, t94468, t94472, t94474, t94477, t94479)
}
