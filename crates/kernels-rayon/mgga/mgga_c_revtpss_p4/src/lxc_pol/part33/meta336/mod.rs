//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta336(t3800: f64, t498: f64, t12295: f64, t1207: f64, t456: f64) -> (f64, f64, f64, f64, f64) {
        let (t12587, t12610, t12625, t12626, t12627) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1347(t3800, t498, t12295, t1207, t456);
    (t12587, t12610, t12625, t12626, t12627)
}
