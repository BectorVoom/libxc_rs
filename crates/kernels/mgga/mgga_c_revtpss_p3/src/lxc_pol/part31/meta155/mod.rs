//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk788;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta155<F: Float>(t225: F, t4075: F, t1429: F, t2435: F, t1428: F, t2777: F, t2439: F, t1385: F, t1398: F, t555: F, t543: F, t2782: F, t1419: F, t545: F, t869: F, t689: F, t136: F, t2457: F, t3964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4076, t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk788::<F>(t225, t4075, t1429, t2435, t1428, t2777, t2439, t1385);
        let (t4089, t4090, t4092, t4093, t4094, t4096, t4099) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk789::<F>(t1398, t555, t4086, t543, t2782, t1419, t545, t869, t689, t136, t2457, t3964);
    (t4076, t4082, t4083, t4085, t4086, t4089, t4090, t4092, t4093, t4094, t4096, t4099)
}
