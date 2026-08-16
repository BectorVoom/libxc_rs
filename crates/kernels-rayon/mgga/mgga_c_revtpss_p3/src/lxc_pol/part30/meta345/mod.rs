//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1359;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta345(t11643: f64, t3127: f64, t3135: f64, t3172: f64, t1041: f64, t1024: f64, t3105: f64, t3151: f64, t3153: f64, t1052: f64, t360: f64, t3089: f64, t1087: f64, t3090: f64, t3278: f64, t3133: f64, t73: f64, t2258: f64, t3094: f64, t3182: f64, t828: f64, t2852: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11644, t11649, t11656, t11659, t11670, t11671) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1359(t11643, t3127, t3135, t3172, t1041, t1024, t3105, t3151, t3153, t1052, t360, t3089);
        let (t11672, t11675, t11678, t11696, t11703, t11704) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1360(t1087, t11671, t3090, t3278, t3133, t73, t2258, t3094, t3182, t828, t2852, t357);
    (t11644, t11649, t11656, t11659, t11670, t11671, t11672, t11675, t11678, t11696, t11703, t11704)
}
