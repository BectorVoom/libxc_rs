//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1408;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta372(t4533: f64, t72: f64, t686: f64, t2465: f64, t1569: f64, t867: f64, t786: f64, t2467: f64, t122: f64, t4480: f64, t2466: f64, t10995: f64, t11044: f64, t4481: f64, t2435: f64, t4477: f64, t136: f64, t1579: f64, t2457: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2444: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14983, t14985, t14987, t14989, t14991, t14992) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1408(t4533, t72, t686, t2465, t1569, t867, t786, t2467, t122, t4480, t2466, t10995);
        let (t14995, t14998, t15003, t15004, t15006, t15008) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1409(t11044, t4481, t2435, t4477, t136, t1579, t2457, t10504, t2471, t4325, t1580, t2444);
    (t14983, t14985, t14987, t14989, t14991, t14992, t14995, t14998, t15003, t15004, t15006, t15008)
}
