//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1455;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta261<F: Float>(t1353: F, t1412: F, t808: F, t9736: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t3946: F, t159: F, t216: F, t124: F, t800: F, t9400: F, t3989: F, t4014: F, t1370: F, t9697: F, t9700: F, t9705: F, t9711: F, t9712: F, t9716: F, t9725: F, t9729: F, t9735: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9737, t9738, t9739, t9741, t9742, t9744) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1455::<F>(t1353, t1412, t808, t9736, t1369, t2699, t1372, t3943, t794);
        let (t9745, t9747, t9748, t9750, t9753, t9755) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1456::<F>(t3946, t9744, t1412, t159, t216, t124, t800, t9400, t3989, t4014, t1370, t9697, t9700, t9705, t9711, t9712, t9716, t9725, t9729, t9735, t9739, t9742);
    (t9737, t9738, t9739, t9741, t9742, t9744, t9745, t9747, t9748, t9750, t9753, t9755)
}
