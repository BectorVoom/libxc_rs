//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta349(t3241: f64, t3244: f64, t1058: f64, t3197: f64, t11132: f64, t3163: f64, t3172: f64, t3161: f64, t126: f64, t373: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11886, t11888, t11890, t11916, t11917, t11921, t11922) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1369(t3241, t3244, t1058, t3197, t11132, t3163, t3172, t3161, t126, t373, t828);
    (t11886, t11888, t11890, t11916, t11917, t11921, t11922)
}
