//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2156;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta520<F: Float>(t3057: F, t379: F, t1078: F, t1651: F, t3066: F, t1695: F, t3325: F, t3269: F, t3270: F, t11121: F, t5015: F, t999: F, t1079: F, t342: F, t4930: F, t1071: F, t1647: F, t3059: F, t1076: F, t1097: F, t11195: F, t1696: F, t3052: F, t3058: F, t3067: F, t3271: F, t3326: F, t4752: F, t4778: F, t4935: F, t5016: F, t995: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16312, t16313, t16314, t16318, t16322, t16327) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2156::<F>(t3057, t379, t1078, t1651, t3066, t1695, t3325, t3269, t3270, t11121, t5015, t999);
        let (t16328, t16333, t16340, t16344, t16352, t16355) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2157::<F>(t1079, t16327, t342, t4930, t1071, t1647, t1695, t3059, t1651, t3325, t1076, t1097, t11195, t16312, t16314, t16318, t16322, t1696, t3052, t3058, t3067, t3271, t3326, t4752, t4778, t4935, t5016, t995);
    (t16312, t16313, t16314, t16318, t16322, t16328, t16333, t16340, t16344, t16352, t16355)
}
