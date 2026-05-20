//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta770 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2571;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta770<F: Float>(t247: F, t44545: F, t5230: F, t5384: F, t12984: F, t5327: F, t17303: F, t3667: F, t12627: F, t489: F, t17728: F, t13011: F, t5373: F, t1222: F, t5368: F, t697: F, t3625: F, t44250: F, t5406: F, t3781: F, t5219: F, t5330: F, t12881: F, t5391: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t57242, t57251, t57257, t57264, t57265, t57270) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2571::<F>(t247, t44545, t5230, t5384, t12984, t5327, t17303, t3667, t12627, t489, t17728, t13011, t5373);
        let (t57271, t57274, t57331, t57382, t57421) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2572::<F>(t57270, t1222, t5368, t697, t3625, t44250, t5406, t3781, t5219, t5330, t12881, t5391);
    (t57242, t57251, t57257, t57264, t57265, t57271, t57274, t57331, t57382, t57421)
}
