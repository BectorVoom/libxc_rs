//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk882;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta137<F: Float>(t3230: F, t351: F, t1054: F, t1058: F, t1014: F, t2857: F, t2251: F, t1012: F, t1010: F, t614: F, t1016: F, t140: F, t1011: F, t1015: F, t2258: F, t271: F, t905: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3231, t3234, t3237, t3238, t3241) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk882::<F>(t3230, t351, t1054, t1058, t1014, t2857, t2251, t1012, t1010, t614);
        let (t3244, t3245, t3247, t3248, t3252) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk883::<F>(t1016, t140, t1011, t1015, t2258, t1012, t271, t905);
    (t3231, t3234, t3237, t3238, t3241, t3244, t3245, t3247, t3248, t3252)
}
