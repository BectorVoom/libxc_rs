//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk911;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk912;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta143<F: Float>(t225: F, t4075: F, t1429: F, t2435: F, t1428: F, t2777: F, t2439: F, t1385: F, t1398: F, t555: F, t543: F) -> (F, F, F, F, F, F) {
        let t4076 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk911::<F>(t225, t4075);
        let (t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk912::<F>(t1429, t2435, t1428, t2777, t2439, t1385, t225);
        let t4089 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk913::<F>(t1398, t555, t4086, t543);
    (t4076, t4082, t4083, t4085, t4086, t4089)
}
