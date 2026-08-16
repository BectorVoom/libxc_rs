//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2284;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2285;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta613<F: Float>(t12382: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t422: F, t17023: F, t17032: F, t17154: F, t24219: F, t24223: F, t24253: F, t24257: F, t24259: F, t24261: F, t24264: F, t24326: F, t24329: F, t24431: F, t24436: F, t24453: F, t3477: F, t3521: F, t435: F, t5120: F, t6487: F, t6503: F, t6506: F, t6519: F, t24428: F, t300: F, t20895: F, t5184: F, t1196: F, t24214: F, t24217: F, t24255: F) -> (F, F, F, F, F, F) {
        let (t24466, t24468) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2284::<F>(t12382, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250, t422);
        let t24470 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2285::<F>(t17023, t17032, t17154, t24219, t24223, t24253, t24257, t24259, t24261, t24264, t24326, t24329, t24431, t24436, t24453, t24468, t3477, t3521, t435, t5120, t6487, t6503, t6506, t6519);
        let (t24472, t24473, t24475, t24476) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2286::<F>(t24428, t24470, t300, t20895, t5184, t1196, t24214, t24217, t24219, t24223, t24255, t24257, t24259, t24261, t24264, t24326, t24329);
    (t24466, t24468, t24472, t24473, t24475, t24476)
}
