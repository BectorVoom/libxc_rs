//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta377(t1134: f64, t3390: f64, t3399: f64, t3407: f64, t12295: f64, t11335: f64, t281: f64, t414: f64, t1139: f64, t12322: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12344, t12347, t12349, t12351, t12352, t12354, t12356) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1788(t1134, t3390, t3399, t3407, t12295, t11335, t281, t414, t1139, t12322, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12344, t12347, t12349, t12351, t12352, t12354, t12356)
}
