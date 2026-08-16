//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1848;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta589(t87071: f64, t92516: f64, t116: f64, t117: f64, t1916: f64, t1918: f64, t22633: f64, t25055: f64, t25063: f64, t25066: f64, t25069: f64, t572: f64, t573: f64, t5801: f64, t5883: f64, t5920: f64, t6941: f64, t6945: f64, t6948: f64, t87051: f64, t87237: f64, param_d: f64, t1458: f64, t1914: f64, t1921: f64, t25049: f64, t25072: f64, t3: f64, t575: f64, t6937: f64, t6951: f64, t75808: f64, t86897: f64, t86903: f64, t86909: f64) -> f64 {
        let (t92517, t92552) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1848(t87071, t92516, t116, t117, t1916, t1918, t22633, t25055, t25063, t25066, t25069, t572, t573, t5801, t5883, t5920, t6941, t6945, t6948, t87051, t87237, param_d);
        let tv4rho44 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1849(t1458, t1914, t1921, t25049, t25072, t3, t575, t6937, t6951, t75808, t86897, t86903, t86909, t92517, t92552);
    tv4rho44
}
