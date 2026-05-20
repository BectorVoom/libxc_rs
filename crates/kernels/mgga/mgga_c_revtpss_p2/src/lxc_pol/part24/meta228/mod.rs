//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta228 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk985;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta228<F: Float>(t1263: F, t675: F, t126: F, t3617: F, t2434: F, t371: F, t482: F, t481: F, t1284: F, t3566: F, t3624: F, t828: F, t12627: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12879, t12884, t12898, t12900, t12909, t12910, t12915, t12916) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk985::<F>(t1263, t675, t126, t3617, t2434, t371, t482, t481, t1284, t3566, t3624, t828);
        let t12987 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk986::<F>(t12627, t225);
    (t12879, t12884, t12898, t12900, t12909, t12910, t12915, t12916, t12987)
}
