//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1033 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3616;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1033(t20343: f64, t698: f64, t20346: f64, t141: f64, t3417: f64, t68355: f64, t12254: f64, t68340: f64, t1134: f64, t5079: f64, t16851: f64, t16854: f64, t58207: f64, t68454: f64, t68529: f64, t68532: f64, t68535: f64, t68538: f64, t68540: f64, t68543: f64, t68546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t68548, t68550, t68553, t68556, t68559, t68561) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3616(t20343, t698, t20346, t141, t3417, t68355, t12254, t68340, t1134, t5079, t16851, t16854);
        let t68564 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3617(t58207, t68454, t68529, t68532, t68535, t68538, t68540, t68543, t68546, t68548, t68550, t68553, t68556, t68559, t68561);
    (t68548, t68550, t68553, t68556, t68559, t68561, t68564)
}
