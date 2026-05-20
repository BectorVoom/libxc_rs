//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1756;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta397<F: Float>(t3704: F, t5293: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F, t12268: F, t3617: F, t3708: F, t5265: F, t1260: F, t5326: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17509, t17524, t17525, t17528, t17529, t17544, t17546, t17547) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1756::<F>(t3704, t5293, t1802, t3147, t3597, t3594, t1244, t3172, t5286, t1247, t3707, t5292);
        let (t17550, t17556, t17569) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1757::<F>(t12268, t3617, t3708, t5265, t1260, t5326);
    (t17509, t17524, t17525, t17528, t17529, t17544, t17546, t17547, t17550, t17556, t17569)
}
