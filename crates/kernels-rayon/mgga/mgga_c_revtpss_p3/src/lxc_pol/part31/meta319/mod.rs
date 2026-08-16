//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta319(t11273: f64, t3160: f64, t2923: f64, t910: f64, t287: f64, t2922: f64, t275: f64, t11132: f64, t240: f64, t624: f64, t281: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11277, t11294, t11299, t11304, t11334, t11335, t11337) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1322(t11273, t3160, t2923, t910, t287, t2922, t275, t11132, t240, t624, t281, t283);
    (t11277, t11294, t11299, t11304, t11334, t11335, t11337)
}
