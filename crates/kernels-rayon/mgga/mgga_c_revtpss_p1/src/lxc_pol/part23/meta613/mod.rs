//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2284;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2285;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta613(t12382: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t422: f64, t17023: f64, t17032: f64, t17154: f64, t24219: f64, t24223: f64, t24253: f64, t24257: f64, t24259: f64, t24261: f64, t24264: f64, t24326: f64, t24329: f64, t24431: f64, t24436: f64, t24453: f64, t3477: f64, t3521: f64, t435: f64, t5120: f64, t6487: f64, t6503: f64, t6506: f64, t6519: f64, t24428: f64, t300: f64, t20895: f64, t5184: f64, t1196: f64, t24214: f64, t24217: f64, t24255: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t24466, t24468) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2284(t12382, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250, t422);
        let t24470 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2285(t17023, t17032, t17154, t24219, t24223, t24253, t24257, t24259, t24261, t24264, t24326, t24329, t24431, t24436, t24453, t24468, t3477, t3521, t435, t5120, t6487, t6503, t6506, t6519);
        let (t24472, t24473, t24475, t24476) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2286(t24428, t24470, t300, t20895, t5184, t1196, t24214, t24217, t24219, t24223, t24255, t24257, t24259, t24261, t24264, t24326, t24329);
    (t24466, t24468, t24472, t24473, t24475, t24476)
}
