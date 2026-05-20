//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta883 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2794;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta883<F: Float>(t22351: F, t2439: F, t2777: F, t22253: F, t4101: F, t686: F, t72: F, t22335: F, t2470: F, t10073: F, t22361: F, t10069: F, t22373: F, t10139: F, t136: F, t2457: F, t6874: F, t6844: F, t14145: F, t14171: F, t1882: F, t2482: F, t22365: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t75074, t75089, t75092, t75113, t75119) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2794::<F>(t22351, t2439, t2777, t22253, t4101, t686, t72, t22335, t2470, t10073, t22361, t10069, t22373);
        let (t75123, t75128, t75141, t75145, t75147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2795::<F>(t10139, t136, t2457, t6874, t6844, t14145, t14171, t1882, t2482, t10069, t22361, t22365);
    (t75074, t75089, t75092, t75113, t75119, t75123, t75128, t75141, t75145, t75147)
}
