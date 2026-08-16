//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta839 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2712;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta839(t1263: f64, t372: f64, t6628: f64, t21233: f64, t3647: f64, t17451: f64, t17605: f64, t17209: f64, t17569: f64, t20824: f64, t3172: f64, t3711: f64, t20879: f64, t1260: f64, t20850: f64, t11262: f64, t3600: f64, t6630: f64, t17225: f64, t5391: f64, t21183: f64, t20875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69839, t69856, t69866, t69885, t69890) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2712(t1263, t372, t6628, t21233, t3647, t17451, t17605, t17209, t17569, t20824, t3172, t3711);
        let (t69899, t69906, t69910, t69916, t69936, t69939) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2713(t20879, t3172, t3711, t1260, t20850, t11262, t3600, t6630, t17225, t5391, t21183, t20875);
    (t69839, t69856, t69866, t69885, t69890, t69899, t69906, t69910, t69916, t69936, t69939)
}
