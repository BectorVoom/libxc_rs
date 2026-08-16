//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta212(t10428: f64, t707: f64, t2398: f64, t2414: f64, t10326: f64, t190: f64, t706: f64, t2258: f64, t750: f64, t157: f64, t36: f64, t10356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10430, t10432, t10433, t10435, t10436, t10438, t10439, t10440) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk993(t10428, t707, t2398, t2414, t10326, t190, t706, t2258, t750, t157, t36, t10356);
    (t10430, t10432, t10433, t10435, t10436, t10438, t10439, t10440)
}
