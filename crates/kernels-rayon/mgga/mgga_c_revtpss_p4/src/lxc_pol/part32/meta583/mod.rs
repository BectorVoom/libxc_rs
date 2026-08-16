//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta583(t98229: f64, t98235: f64, t98238: f64, t98243: f64, t98258: f64, t98269: f64, t98281: f64, t1904: f64, t2439: f64, t26358: f64, t213: f64, t28888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102531, t102534, t102535, t102537, t102548, t102557, t102567, t102582, t102594) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1911(t98229, t98235, t98238, t98243, t98258, t98269, t98281, t1904, t2439, t26358, t213, t28888);
    (t102531, t102534, t102535, t102537, t102548, t102557, t102567, t102582, t102594)
}
