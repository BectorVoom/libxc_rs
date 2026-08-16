//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1308;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1309;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta362(t15008: f64, t689: f64, t213: f64, t4469: f64, t1580: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64, t4321: f64, t887: f64, t4470: f64, t786: f64, t789: f64, t4534: f64, t779: f64, t2435: f64, t4322: f64, t1596: f64, t2873: f64, t1614: f64, t2942: f64, t1606: f64, t4580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15010, t15011, t15015, t15018, t15045) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1308(t15008, t689, t213, t4469, t1580, t2440, t2439, t1569, t2453, t2458, t4321, t887);
        let (t15047, t15050, t15062, t15063, t15101, t15104) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1309(t15045, t689, t4470, t786, t789, t4534, t779, t2435, t4322, t1596, t2873, t1614, t2942);
        let (t15123, t15125) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1310(t1606, t2439, t4580, t689);
    (t15010, t15011, t15015, t15018, t15047, t15050, t15062, t15063, t15101, t15104, t15123, t15125)
}
