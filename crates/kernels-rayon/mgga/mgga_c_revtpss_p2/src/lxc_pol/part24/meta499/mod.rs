//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1501;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta499(t14598: f64, t23160: f64, t686: f64, t72: f64, t23244: f64, t251: f64, t1568: f64, t5977: f64, t2723: f64, t2782: f64, t4503: f64, t1558: f64, t6041: f64, t231: f64, t2783: f64, t4500: f64, t62967: f64, t23168: f64, t39598: f64, t10530: f64, t23172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76125, t76127, t76131, t76134, t76136) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1501(t14598, t23160, t686, t72, t23244, t251, t1568, t5977, t2723, t2782, t4503, t1558, t6041);
        let (t76139, t76144, t76153, t76158) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1502(t231, t2782, t2783, t76136, t4500, t62967, t23168, t39598, t686, t72, t10530, t23172);
    (t76125, t76127, t76131, t76134, t76139, t76144, t76153, t76158)
}
