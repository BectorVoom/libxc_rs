//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1493;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta494<F: Float>(t14220: F, t48007: F, t22331: F, t2470: F, t4101: F, t10073: F, t22369: F, t136: F, t2457: F, t47429: F, t6862: F, t22351: F, t2439: F, t2777: F, t22335: F, t22361: F, t10069: F, t22373: F, t10139: F, t6874: F, t6844: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t75005, t75021, t75026, t75068, t75074) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1493::<F>(t14220, t48007, t22331, t2470, t4101, t10073, t22369, t136, t2457, t47429, t6862, t22351, t2439, t2777);
        let (t75092, t75113, t75119, t75123, t75128) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1494::<F>(t22335, t2470, t4101, t10073, t22361, t10069, t22373, t10139, t136, t2457, t6874, t6844);
    (t75005, t75021, t75026, t75068, t75074, t75092, t75113, t75119, t75123, t75128)
}
