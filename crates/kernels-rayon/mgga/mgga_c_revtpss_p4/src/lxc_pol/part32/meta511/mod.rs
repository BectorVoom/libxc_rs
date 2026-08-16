//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta511(t233: f64, t30379: f64, t1957: f64, t225: f64, t2061: f64, t5977: f64, t2723: f64, t25416: f64, t231: f64, t7076: f64, t1558: f64, t7997: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30380, t30381, t30384, t30391, t30392, t30395, t30396, t30400) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1804(t233, t30379, t1957, t225, t2061, t5977, t2723, t25416, t231, t7076, t1558, t7997);
    (t30380, t30381, t30384, t30391, t30392, t30395, t30396, t30400)
}
