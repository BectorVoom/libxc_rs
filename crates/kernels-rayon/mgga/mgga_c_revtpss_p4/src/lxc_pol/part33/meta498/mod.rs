//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1804;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta498(t25304: f64, t7283: f64, t25946: f64, t25949: f64, t786: f64, t7286: f64, t1426: f64, t3999: f64, t213: f64, t7274: f64, t116: f64, t7002: f64, t10301: f64, t7565: f64, t38: f64, t7574: f64, t2247: f64, t2282: f64, t55: f64, t10309: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26069, t26071, t26072, t26073, t26079, t26084, t26123) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1804(t25304, t7283, t25946, t25949, t786, t7286, t1426, t3999, t213, t7274, t116, t7002);
        let (t26749, t26754, t26755, t26776, t26792) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1805(t10301, t7565, t38, t7574, t2247, t2282, t55, t10309);
    (t26069, t26071, t26072, t26073, t26079, t26084, t26123, t26749, t26754, t26755, t26776, t26792)
}
