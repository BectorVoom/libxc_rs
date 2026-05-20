//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1470;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta481<F: Float>(t17376: F, t17524: F, t17528: F, t3140: F, t6564: F, t3599: F, t17361: F, t5274: F, t1234: F, t21271: F, t21093: F, t372: F, t1263: F, t6628: F, t1260: F, t20850: F, t11262: F, t3600: F, t6630: F, t3610: F, t6634: F, t5326: F, t5390: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t69680, t69683, t69692, t69693, t69700, t69795, t69832) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1470::<F>(t17376, t17524, t17528, t3140, t6564, t3599, t17361, t5274, t1234, t21271, t21093, t372);
        let (t69839, t69906, t69910, t69964, t69968) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1471::<F>(t1263, t372, t6628, t1260, t20850, t11262, t3600, t6630, t3610, t6634, t5326, t5390);
    (t69680, t69683, t69692, t69693, t69700, t69795, t69832, t69839, t69906, t69910, t69964, t69968)
}
