//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1474;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta407<F: Float>(t15559: F, t981: F, t3336: F, t5019: F, t11108: F, t1699: F, t3022: F, t4725: F, t11465: F, t1633: F, t3015: F, t3026: F, t4719: F, t1695: F, t3075: F, t1079: F, t3215: F, t4858: F, t372: F, t4872: F, t4786: F, t4873: F, t11696: F, t4781: F, t3092: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15561, t15562, t15566, t15571, t15575, t15577) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1474::<F>(t15559, t981, t3336, t5019, t11108, t1699, t3022, t4725, t11465, t1633, t3015, t3026, t4719);
        let (t15579, t15583, t15586, t15592) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1475::<F>(t1695, t3075, t1079, t3215, t4858, t372, t4872, t4786, t4873, t11696, t4781, t3092);
    (t15561, t15562, t15566, t15571, t15575, t15577, t15579, t15583, t15586, t15592)
}
