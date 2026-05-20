//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1167;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1168;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta295<F: Float>(t12254: F, t12257: F, t141: F, t1146: F, t2439: F, t3424: F, t698: F, t3421: F, t3361: F, t57: F, t10356: F, t3417: F, t3362: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12258, t12259, t12261, t12263, t12265, t12267, t12268) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1167::<F>(t12254, t12257, t141, t1146, t2439, t3424, t698, t3421, t3361, t57);
        let t12269 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1168::<F>(t10356, t12268);
        let (t12270, t12271, t12273) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1169::<F>(t12269, t3417, t141, t10356, t3362);
    (t12258, t12259, t12261, t12263, t12265, t12267, t12268, t12269, t12270, t12271, t12273)
}
