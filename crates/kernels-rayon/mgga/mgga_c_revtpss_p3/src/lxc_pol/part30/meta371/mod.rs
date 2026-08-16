//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta371(t1222: f64, t13011: f64, t140: f64, t3688: f64, t3700: f64, t3367: f64, t404: f64, t1242: f64, t3603: f64, t471: f64, t1032: f64, t3552: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13012, t13014, t13015, t13017, t13018, t13026, t13038, t13045, t13068) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1398(t1222, t13011, t140, t3688, t3700, t3367, t404, t1242, t3603, t471, t1032, t3552);
    (t13012, t13014, t13015, t13017, t13018, t13026, t13038, t13045, t13068)
}
