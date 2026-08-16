//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1315;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta319(t10509: f64, t123: f64, t2465: f64, t213: f64, t2760: f64, t215: f64, t231: f64, t268: f64, t836: f64, t2798: f64, t2722: f64, t675: f64, t251: f64, t4503: f64, t786: f64, t2723: f64, t2453: f64, t2797: f64, t281: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10510, t10511, t10513, t10519, t10521) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1315(t10509, t123, t2465, t213, t2760, t215, t231, t268, t836, t2798, t2722, t675);
        let (t10524, t10529, t10533, t10535, t10538) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1316(t10521, t231, t268, t2798, t251, t4503, t786, t2723, t2453, t2797, t281, t68, t836);
    (t10510, t10511, t10513, t10519, t10524, t10529, t10533, t10535, t10538)
}
