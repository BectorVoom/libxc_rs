//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1655;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta270(t3: f64, t6936: f64, t116: f64, t5883: f64, t117: f64, t5920: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t640: f64, t76: f64, param_d: f64, t159: f64, t793: f64, t1518: f64, t94: f64, t93: f64, t587: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6937, t6941, t6945, t6948, t6951, t6977) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1655(t3, t6936, t116, t5883, t117, t5920, t1916, t1918, t572, t573, t640, t76, param_d);
        let (t7021, t7732, t7889, t8779) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1656(t159, t793, t1518, t94, t93, t587, t65);
    (t6937, t6941, t6945, t6948, t6951, t6977, t7021, t7732, t7889, t8779)
}
