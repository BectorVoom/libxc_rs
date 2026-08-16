//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta636(t100690: f64, t994: f64, t7150: f64, t7810: f64, t989: f64, t25698: f64, t27418: f64, t4746: f64, t7135: f64, t1982: f64, t99708: f64, t3047: f64, t8521: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100691, t100698, t100702, t100705, t100708, t100723, t100737) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2091(t100690, t994, t7150, t7810, t989, t25698, t27418, t4746, t7135, t1982, t99708, t3047, t8521);
    (t100691, t100698, t100702, t100705, t100708, t100723, t100737)
}
