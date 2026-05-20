//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta740 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2518;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2519;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta740<F: Float>(t2475: F, t808: F, t11028: F, t1580: F, t2439: F, t10504: F, t15002: F, t9285: F, t10505: F, t137: F, t41011: F, t11015: F, t4325: F, t4477: F, t9292: F, t14472: F, t887: F, t11044: F, t14485: F, t15014: F, t9303: F, t10510: F, t14987: F, t10982: F, t1568: F, t9646: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51176, t51199, t51203, t51208, t51211) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2518::<F>(t2475, t808, t11028, t1580, t2439, t10504, t15002, t9285, t10505, t137, t41011, t11015, t4325);
        let (t51213, t51217, t51234, t51237, t51240, t51246) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2519::<F>(t4477, t9292, t14472, t2439, t887, t11044, t14485, t15014, t9303, t10510, t14987, t10982, t1568, t9646);
    (t51176, t51199, t51203, t51208, t51211, t51213, t51217, t51234, t51237, t51240, t51246)
}
