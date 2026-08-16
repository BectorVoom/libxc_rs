//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk854;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk855;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk856;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta212(t4729: f64, t981: f64, t1633: f64, t3011: f64, t3014: f64, t972: f64, t2848: f64, t3037: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t341: f64, t1646: f64, t993: f64, t378: f64, t1647: f64, t1651: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4731, t4732, t4733, t4734, t4736, t4742) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk854(t4729, t981, t1633, t3011, t3014, t972, t2848, t3037, t4571, t4576, t4581, t4585);
        let t4743 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk855(t341, t4742);
        let t4746 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk856(t1646, t993);
        let (t4747, t4752, t4757) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk857(t378, t4746, t1647, t1651, t999);
    (t4731, t4732, t4733, t4734, t4736, t4742, t4743, t4746, t4747, t4752, t4757)
}
