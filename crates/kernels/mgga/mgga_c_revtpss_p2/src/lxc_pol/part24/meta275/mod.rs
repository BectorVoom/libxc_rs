//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1048;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta275<F: Float>(t177: F, t5941: F, t762: F, t1553: F, t73: F, t2475: F, t5966: F, t5962: F, t853: F, t221: F, t2485: F, t6017: F, t2484: F, t125: F, t10779: F, t14671: F, t6035: F, t10777: F, t251: F, t5977: F, t1558: F, t1568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18562, t18563, t18592, t18599, t18608, t18622) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1048::<F>(t177, t5941, t762, t1553, t73, t2475, t5966, t5962, t853, t221, t2485, t6017);
        let (t18623, t18627, t18643, t18644, t18677, t18681) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1049::<F>(t18622, t2484, t125, t5962, t10779, t14671, t6035, t10777, t251, t5977, t1558, t1568);
    (t18562, t18563, t18592, t18599, t18608, t18622, t18623, t18627, t18643, t18644, t18677, t18681)
}
