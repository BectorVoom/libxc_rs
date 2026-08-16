//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1713;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta461(t2470: f64, t7514: f64, t7284: f64, t25878: f64, t26234: f64, t1445: f64, t7492: f64, t689: f64, t1385: f64, t2097: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t26292 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1713(t2470, t7514);
        let (t26294, t26295, t26301, t26302, t26304) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1714(t26292, t7284, t25878, t26234, t1445, t7492, t689, t1385, t2097);
    (t26292, t26294, t26295, t26301, t26302, t26304)
}
