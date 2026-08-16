//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta378<F: Float>(t12334: F, t12356: F, t1150: F, t1131: F, t1126: F, t3383: F, t3386: F, t12228: F, t3433: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12357, t12358, t12360, t12361, t12363, t12364, t12366, t12367, t12378) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1789::<F>(t12334, t12356, t1150, t1131, t1126, t3383, t3386, t12228, t3433, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12357, t12358, t12360, t12361, t12363, t12364, t12366, t12367, t12378)
}
