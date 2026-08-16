//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1321;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta318(t3145: f64, t334: f64, t368: f64, t3153: f64, t73: f64, t246: f64, t676: f64, t1046: f64, t1041: f64, t3140: f64, t989: f64, t3149: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11243, t11249, t11262, t11263, t11264, t11273, t11274) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1321(t3145, t334, t368, t3153, t73, t246, t676, t1046, t1041, t3140, t989, t3149);
    (t11243, t11249, t11262, t11263, t11264, t11273, t11274)
}
