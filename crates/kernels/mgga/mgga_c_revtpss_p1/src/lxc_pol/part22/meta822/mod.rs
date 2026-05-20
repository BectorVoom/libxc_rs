//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2937;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta822<F: Float>(t14202: F, t9303: F, t14238: F, t2453: F, t10142: F, t10019: F, t14239: F, t1882: F, t4066: F, t1398: F, t21990: F, t10022: F, t2782: F, t13790: F, t4056: F, t10073: F, t14231: F, t10139: F, t14219: F, t9285: F, t14215: F, t2470: F, t4101: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48005, t48007, t48008, t48013, t48015, t48020, t48022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2937::<F>(t14202, t9303, t14238, t2453, t10142, t10019, t14239, t1882, t4066, t1398, t21990, t10022, t2782);
        let (t48027, t48029, t48036, t48039) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2938::<F>(t13790, t4056, t10022, t2782, t10073, t14231, t10139, t14219, t9285, t14215, t2470, t4101);
    (t48005, t48007, t48008, t48013, t48015, t48020, t48022, t48027, t48029, t48036, t48039)
}
