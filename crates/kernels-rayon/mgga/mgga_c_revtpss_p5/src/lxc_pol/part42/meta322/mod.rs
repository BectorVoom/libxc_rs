//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1102;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta322(t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64, t2470: f64, t5721: f64, t3915: f64, t1445: f64, t5599: f64, t2435: f64, t5600: f64, t1426: f64, t1893: f64, t3917: f64, t136: f64, t1903: f64, t2457: f64, t9674: f64, t10175: f64, t5722: f64, t122: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14084, t14087, t14091, t14096, t14097) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1102(t5711, t786, t1364, t1357, t5775, t689, t2470, t5721, t3915, t1445, t5599, t2435, t5600);
        let (t14100, t14102, t14105, t14108, t14109) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1103(t1426, t1893, t786, t3917, t136, t1903, t2457, t9674, t10175, t5722, t122, t5721);
    (t14084, t14087, t14091, t14096, t14097, t14100, t14102, t14105, t14108, t14109)
}
