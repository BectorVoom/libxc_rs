//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta272(t240: f64, t4000: f64, t532: f64, t549: f64, t72: f64, t595: f64, t66: f64, t247: f64, t550: f64, t548: f64, t4010: f64, t245: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t9934, t9942, t9949, t9953, t9954, t9955) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1021(t240, t4000, t532, t549, t72, t595, t66, t247, t550, t548, t4010, t245);
    (t9934, t9942, t9949, t9953, t9954, t9955)
}
