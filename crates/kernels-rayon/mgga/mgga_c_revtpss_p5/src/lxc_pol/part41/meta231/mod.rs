//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk893;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk894;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta231(t1651: f64, t996: f64, t1695: f64, t1079: f64, t3070: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t1592: f64, t4823: f64, t1042: f64, t1469: f64, t3094: f64, t4781: f64, t3092: f64, t1668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6244 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk893(t1651);
        let (t6245, t6251, t6258) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk894(t6244, t996, t1651, t1695, t1079, t3070, t4571, t6094, t6098, t6102);
        let (t6259, t6262, t6263, t6266, t6267, t6268, t6271) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk895(t6258, t996, t1592, t4823, t1042, t1469, t3094, t4781, t3092, t1651, t1668);
    (t6244, t6245, t6251, t6258, t6259, t6262, t6263, t6266, t6267, t6268, t6271)
}
