//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2397;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta666<F: Float>(t1021: F, t11970: F, t11874: F, t15688: F, t11853: F, t828: F, t3181: F, t675: F, t283: F, t2852: F, t11144: F, t3252: F, t11852: F, t126: F, t12166: F, t15905: F, t994: F, t11631: F, t999: F, t3046: F, t3298: F, t4891: F, t1052: F, t11243: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42326, t42328, t42410, t42447, t42471, t42518) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2397::<F>(t1021, t11970, t11874, t15688, t11853, t828, t3181, t675, t283, t2852, t11144, t3252);
        let (t42534, t42621, t42622, t42643, t42646) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2398::<F>(t11852, t126, t12166, t15905, t994, t11631, t999, t3046, t3298, t4891, t1052, t11243);
    (t42326, t42328, t42410, t42447, t42471, t42518, t42534, t42621, t42622, t42643, t42646)
}
