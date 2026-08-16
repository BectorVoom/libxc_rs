//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1649;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta439(t4003: f64, t5658: f64, t1448: f64, t1868: f64, t197: f64, t531: f64, t2013: f64, t1450: f64, t3889: f64, t2242: f64, t607: f64, t640: f64, t644: f64, t77: f64, t2315: f64, t84: f64, t2251: f64, t603: f64, t2259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21990, t22496, t25081, t25082) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1649(t4003, t5658, t1448, t1868, t197, t531, t2013);
        let (t25089, t25102, t25110, t25114, t25117, t25120) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1650(t1450, t3889, t2242, t607, t640, t644, t77, t2315, t84, t2251, t603, t2259);
    (t21990, t22496, t25081, t25082, t25089, t25102, t25110, t25114, t25117, t25120)
}
