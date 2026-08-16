//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2500;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta731(t1531: f64, t36: f64, t14362: f64, t9863: f64, t9866: f64, t2609: f64, t4395: f64, t10115: f64, t1570: f64, t11007: f64, t1579: f64, t4322: f64, t9292: f64, t10981: f64, t22: f64, t868: f64, t15060: f64, t2435: f64, t14982: f64, t2465: f64, t2470: f64, t4480: f64, t9288: f64, t1569: f64, t2769: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50089, t50092, t50094, t50098, t50155, t50161, t50166) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2500(t1531, t36, t14362, t9863, t9866, t2609, t4395, t10115, t1570, t11007, t1579, t4322, t9292);
        let (t50178, t50184, t50187, t50205, t50208) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2501(t10981, t1579, t22, t868, t15060, t2435, t14982, t2465, t2470, t4480, t9288, t1569, t2769, t786);
    (t50089, t50092, t50094, t50098, t50155, t50161, t50166, t50178, t50184, t50187, t50205, t50208)
}
