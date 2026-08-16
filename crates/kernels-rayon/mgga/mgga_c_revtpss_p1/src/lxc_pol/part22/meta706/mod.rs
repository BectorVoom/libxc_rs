//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta706(t21660: f64, t22531: f64, t3: f64, t5883: f64, t670: f64, t4292: f64, t5801: f64, t116: f64, t5920: f64, t117: f64, t21881: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t6941: f64, t6945: f64, t6948: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22532, t22533, t22544, t22556, t22559, t22564, t22565, t22568, t22571) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2729(t21660, t22531, t3, t5883, t670, t4292, t5801, t116, t5920, t117, t21881, t1459, t1461, t1916, t1918, t572, t573, t5795, t5802, t5805, t6941, t6945, t6948, param_d);
    (t22532, t22533, t22544, t22556, t22559, t22564, t22565, t22568, t22571)
}
