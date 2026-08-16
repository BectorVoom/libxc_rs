//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta527(t5627: f64, t8996: f64, t28167: f64, t531: f64, t7933: f64, t7238: f64, t2014: f64, t1450: f64, t5591: f64, t7237: f64, t13648: f64, t2034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28168, t28170, t28172, t28173, t28175, t28176, t28177, t28179, t28182) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1941(t5627, t8996, t28167, t531, t7933, t7238, t2014, t1450, t5591, t7237, t13648, t2034);
    (t28168, t28170, t28172, t28173, t28175, t28176, t28177, t28179, t28182)
}
