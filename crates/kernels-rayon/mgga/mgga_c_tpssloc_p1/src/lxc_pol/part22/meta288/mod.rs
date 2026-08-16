//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1442;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta288(t13278: f64, t831: f64, t2629: f64, t4166: f64, t4250: f64, t9638: f64, t4240: f64, t4191: f64, t2697: f64, t4261: f64, t820: f64, t9645: f64, t1484: f64, t828: f64, t1516: f64, t9993: f64, t2696: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13280, t13283, t13287, t13320, t13330, t13345, t13350) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1442(t13278, t831, t2629, t4166, t4250, t9638, t4240, t4191, t2697, t4261, t820, t9645);
        let (t13351, t13359, t13360) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1443(t1484, t828, t1516, t9993, t2696, t4166);
    (t13280, t13283, t13287, t13320, t13330, t13345, t13350, t13351, t13359, t13360)
}
