//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta249(t136: f64, t1579: f64, t2457: f64, t10504: f64, t2471: f64, t4325: f64, t1580: f64, t2440: f64, t2439: f64, t1569: f64, t2453: f64, t2458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15002, t15003, t15004, t15006, t15014, t15015, t15017, t15018) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1012(t136, t1579, t2457, t10504, t2471, t4325, t1580, t2440, t2439, t1569, t2453, t2458);
    (t15002, t15003, t15004, t15006, t15014, t15015, t15017, t15018)
}
