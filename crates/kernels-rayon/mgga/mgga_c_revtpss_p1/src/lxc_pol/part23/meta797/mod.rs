//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta797 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2620;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta797(t10811: f64, t18639: f64, t10905: f64, t18507: f64, t10777: f64, t10779: f64, t2749: f64, t61715: f64, t18651: f64, t14923: f64, t18456: f64, t14671: f64, t14686: f64, t14931: f64, t18632: f64, t4424: f64, t61956: f64, t837: f64, t18477: f64, t50769: f64, t51133: f64, t18348: f64, t2710: f64, t2713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62162, t62168, t62176, t62178, t62188, t62216) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2620(t10811, t18639, t10905, t18507, t10777, t10779, t2749, t61715, t18651, t14923, t18456, t14671, t14686, t14931, t18632);
        let (t62236, t62241, t62246, t62251) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2621(t10777, t14671, t14686, t4424, t61956, t837, t18477, t50769, t51133, t18348, t2710, t2713);
    (t62162, t62168, t62176, t62178, t62188, t62216, t62236, t62241, t62246, t62251)
}
