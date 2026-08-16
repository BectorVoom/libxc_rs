//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta811 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2914;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta811(t39515: f64, t4083: f64, t10043: f64, t9303: f64, t10139: f64, t281: f64, t4056: f64, t543: f64, t68: f64, t14192: f64, t555: f64, t10115: f64, t1441: f64, t4093: f64, t9292: f64, t10065: f64, t10073: f64, t1432: f64, t1433: f64, t39497: f64, t10061: f64, t10069: f64, t10111: f64, t1428: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47351, t47352, t47364, t47371, t47381) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2914(t39515, t4083, t10043, t9303, t10139, t281, t4056, t543, t68, t14192, t555, t10115, t1441);
        let (t47389, t47391, t47395, t47403, t47413, t47417) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2915(t4093, t9292, t10065, t10073, t1432, t1433, t39497, t10061, t10069, t10111, t1428, t588);
    (t47351, t47352, t47364, t47371, t47381, t47389, t47391, t47395, t47403, t47413, t47417)
}
