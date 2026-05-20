//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1731;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta300<F: Float>(t4021: F, t9976: F, t1398: F, t1412: F, t3938: F, t3992: F, t2661: F, t1384: F, t235: F, t4003: F, t543: F, t2482: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F, t1419: F, t4086: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9977, t9981, t9982, t9989, t9990, t9991, t9994) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1731::<F>(t4021, t9976, t1398, t1412, t3938, t3992, t2661, t1384, t235, t4003, t543);
        let (t10001, t10003, t10004, t10013, t10014) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1732::<F>(t2482, t27, t4000, t221, t4004, t4019, t1419, t4086, t786);
    (t9977, t9981, t9982, t9989, t9990, t9991, t9994, t10001, t10003, t10004, t10013, t10014)
}
