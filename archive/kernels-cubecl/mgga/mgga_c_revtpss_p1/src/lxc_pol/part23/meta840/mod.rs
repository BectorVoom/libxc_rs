//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta840 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2714;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta840<F: Float>(t17217: F, t17505: F, t1032: F, t1246: F, t21333: F, t17720: F, t5391: F, t11262: F, t3610: F, t6634: F, t17569: F, t5326: F, t5390: F, t17361: F, t5293: F, t1261: F, t20863: F, t3172: F, t20973: F, t3647: F, t21242: F, t3636: F, t17306: F, t17728: F, t489: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t69947, t69958, t69961, t69964, t69966, t69968) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2714::<F>(t17217, t17505, t1032, t1246, t21333, t17720, t5391, t11262, t3610, t6634, t17569, t5326, t5390);
        let (t69971, t69984, t70006, t70008, t70014) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2715::<F>(t17361, t5293, t1261, t20863, t3172, t20973, t3647, t21242, t3636, t17306, t17728, t489);
    (t69947, t69958, t69961, t69964, t69966, t69968, t69971, t69984, t70006, t70008, t70014)
}
