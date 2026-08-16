//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk460;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta74(t1916: f64, t1918: f64, t572: f64, t573: f64, t76: f64, t84: f64, t198: f64, t207: f64, t159: f64, t215: f64, t10: f64, t17: f64, t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64, t584: f64, t588: f64, t20: f64, t27: f64, t12: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1921, t1927, t1940, t1941, t2219) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk460(t1916, t1918, t572, t573, t76, t84, t198, t207, t159, t215, t10, t17);
        let (t2221, t2223, t2224, t2226, t2228, t2230, t2231) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk461(t576, t580, t15, t22, t11, t14, t584, t588, t20, t27, t12, t19);
    (t1921, t1927, t1940, t1941, t2219, t2221, t2223, t2224, t2226, t2228, t2230, t2231)
}
