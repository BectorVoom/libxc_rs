//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2829;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta754<F: Float>(t11273: F, t11998: F, t1062: F, t11782: F, t11853: F, t828: F, t3229: F, t360: F, t3089: F, t1087: F, t1024: F, t12003: F, t3181: F, t675: F, t1063: F, t247: F, t2853: F, t283: F, t2852: F, t1025: F, t3218: F, t371: F, t676: F, t11144: F, t3252: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42371, t42391, t42410, t42416, t42417, t42425) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2829::<F>(t11273, t11998, t1062, t11782, t11853, t828, t3229, t360, t3089, t1087, t1024, t12003);
        let (t42447, t42450, t42471, t42481, t42518) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2830::<F>(t3181, t675, t1063, t247, t2853, t283, t2852, t1025, t3218, t371, t676, t11144, t3252);
    (t42371, t42391, t42410, t42416, t42417, t42425, t42447, t42450, t42471, t42481, t42518)
}
