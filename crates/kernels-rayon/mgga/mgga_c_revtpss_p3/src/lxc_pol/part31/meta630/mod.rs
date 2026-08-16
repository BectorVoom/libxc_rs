//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta630(t25569: f64, t4817: f64, t1659: f64, t25576: f64, t27489: f64, t3111: f64, t11940: f64, t7131: f64, t16158: f64, t7132: f64, t100007: f64, t16094: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t100097, t100114, t100117, t100121, t100132, t100135) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2084(t25569, t4817, t1659, t25576, t27489, t3111, t11940, t7131, t16158, t7132, t100007, t16094);
    (t100097, t100114, t100117, t100121, t100132, t100135)
}
