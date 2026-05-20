//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta405<F: Float>(t14100: F, t3917: F, t136: F, t1903: F, t2457: F, t9674: F, t10175: F, t5722: F, t122: F, t5721: F, t3916: F, t9680: F, t1437: F, t1882: F, t2482: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14102, t14103, t14104, t14105, t14108, t14109, t14110, t14111) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1999::<F>(t14100, t3917, t136, t1903, t2457, t9674, t10175, t5722, t122, t5721, t3916, t9680);
        let (t14113, t14114) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2000::<F>(t1437, t1882, t2482);
    (t14102, t14103, t14104, t14105, t14108, t14109, t14110, t14111, t14113, t14114)
}
