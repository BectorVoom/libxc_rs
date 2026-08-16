//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk858;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk859;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta213(t4757: f64, t996: f64, t1096: f64, t1651: f64, t1079: f64, t2848: f64, t3070: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t1678: f64, t994: f64, t1668: f64, t73: f64, t3095: f64, t3092: f64, t3093: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4758, t4764, t4772) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk858(t4757, t996, t1096, t1651, t1079, t2848, t3070, t4571, t4576, t4581, t4585);
        let (t4773, t4778, t4781) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk859(t4772, t996, t1678, t994, t1668, t73);
        let (t4782, t4783, t4786) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk860(t3095, t4781, t3092, t3093, t357);
    (t4758, t4764, t4772, t4773, t4778, t4781, t4782, t4783, t4786)
}
