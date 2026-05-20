//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta857 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta857<F: Float>(t3766: F, t6564: F, t17191: F, t5219: F, t21342: F, t473: F, t1770: F, t17845: F, t17852: F, t17948: F, t13147: F, t1811: F, t460: F) -> (F, F, F, F, F, F, F) {
        let (t72370, t72386, t72397, t72429, t72432, t72435, t72686) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2747::<F>(t3766, t6564, t17191, t5219, t21342, t473, t1770, t17845, t17852, t17948, t13147, t1811, t460);
    (t72370, t72386, t72397, t72429, t72432, t72435, t72686)
}
