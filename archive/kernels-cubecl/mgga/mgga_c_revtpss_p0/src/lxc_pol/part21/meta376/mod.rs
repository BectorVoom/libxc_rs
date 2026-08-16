//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1786;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta376<F: Float>(t12292: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t1132: F, t409: F, t416: F, t1134: F, t3391: F, t406: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F) -> (F, F, F, F, F, F, F, F) {
        let (t12322, t12323) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1786::<F>(t12292, t12296, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t1132);
        let (t12327, t12328, t12329, t12331, t12332, t12334) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1787::<F>(t409, t416, t1134, t3391, t406, t12252, t12259, t12261, t12263, t12265, t12271, t12275, t12279, t12284, t12289, t12292, t12323);
    (t12322, t12323, t12327, t12328, t12329, t12331, t12332, t12334)
}
