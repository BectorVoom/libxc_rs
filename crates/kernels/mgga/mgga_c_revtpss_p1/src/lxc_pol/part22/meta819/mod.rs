//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta819 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2931;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2932;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta819<F: Float>(t136: F, t2457: F, t5774: F, t9674: F, t10175: F, t14079: F, t10073: F, t13731: F, t3915: F, t5721: F, t9288: F, t2439: F, t3895: F, t5775: F, t14066: F, t213: F, t14109: F, t47603: F, t9681: F, t14268: F, t686: F, t72: F, t14293: F, t9664: F, t1444: F, t2782: F, t4075: F, t556: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47885, t47893, t47899, t47904, t47907) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2931::<F>(t136, t2457, t5774, t9674, t10175, t14079, t10073, t13731, t3915, t5721, t9288, t2439, t3895, t5775);
        let (t47909, t47913, t47918, t47920, t47926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2932::<F>(t14066, t213, t14109, t47603, t9681, t14268, t3915, t686, t72, t14293, t9664, t1444, t2782, t4075, t556, t5774);
    (t47885, t47893, t47899, t47904, t47907, t47909, t47913, t47918, t47920, t47926)
}
