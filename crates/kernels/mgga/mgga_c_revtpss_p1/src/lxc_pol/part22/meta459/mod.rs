//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2133;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta459<F: Float>(t1614: F, t2967: F, t1626: F, t2986: F, t4587: F, t914: F, t936: F, t2919: F, t4590: F, t1596: F, t2923: F, t2927: F, t11289: F, t1610: F, t2869: F, t4632: F, t15125: F, t15168: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15163: F, t15166: F, t15170: F, t15173: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15406, t15413, t15416, t15418, t15420, t15421) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2133::<F>(t1614, t2967, t1626, t2986, t4587, t914, t936, t2919, t4590, t1596, t2923);
        let (t15423, t15425, t15427, t15435, t15447, t15450) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2134::<F>(t15421, t2927, t11289, t1610, t2869, t4632, t15125, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
    (t15406, t15413, t15416, t15418, t15420, t15421, t15423, t15425, t15427, t15435, t15447, t15450)
}
