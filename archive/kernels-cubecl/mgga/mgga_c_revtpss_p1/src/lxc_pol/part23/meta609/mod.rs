//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta609<F: Float>(t24232: F, t3417: F, t141: F, t1145: F, t24240: F, t24248: F, t24236: F, t12296: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24288, t24289, t24291, t24292, t24294, t24295, t24297, t24298, t24312) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2273::<F>(t24232, t3417, t141, t1145, t24240, t24248, t24236, t12296, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24288, t24289, t24291, t24292, t24294, t24295, t24297, t24298, t24312)
}
