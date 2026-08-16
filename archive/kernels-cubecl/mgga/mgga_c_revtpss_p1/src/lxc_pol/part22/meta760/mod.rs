//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2840;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta760<F: Float>(t11670: F, t11772: F, t3114: F, t11773: F, t11926: F, t11858: F, t15688: F, t16102: F, t3155: F, t12077: F, t15905: F, t994: F, t3075: F, t3154: F, t11671: F, t11865: F, t11725: F, t828: F, t11660: F, t2258: F, t3204: F, t3230: F, t225: F, t42059: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43065, t43066, t43069, t43082, t43085, t43105) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2840::<F>(t11670, t11772, t3114, t11773, t11926, t11858, t15688, t16102, t3155, t12077, t15905, t994);
        let (t43116, t43121, t43131, t43139, t43151, t43154) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2841::<F>(t3075, t3154, t11671, t11865, t11725, t828, t11660, t2258, t3204, t3230, t225, t42059);
    (t43065, t43066, t43069, t43082, t43085, t43105, t43116, t43121, t43131, t43139, t43151, t43154)
}
