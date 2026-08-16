//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta495(t26120: f64, t572: f64, t116: f64, t7002: f64, t670: f64, t2371: f64, t7330: f64, t117: f64, t25832: f64, t10301: f64, t7565: f64, t38: f64, t7574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26122, t26123, t26124, t26126, t26127, t26129, t26130, t26132, t26749, t26754) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1847(t26120, t572, t116, t7002, t670, t2371, t7330, t117, t25832, t10301, t7565, t38, t7574);
    (t26122, t26123, t26124, t26126, t26127, t26129, t26130, t26132, t26749, t26754)
}
