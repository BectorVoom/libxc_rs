//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2443;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta655<F: Float>(t11853: F, t828: F, t3229: F, t360: F, t3089: F, t1087: F, t11672: F, t11711: F, t1024: F, t12003: F, t11744: F, t3188: F, t3181: F, t675: F, t1063: F, t247: F, t2853: F, t11151: F, t11725: F, t283: F, t2852: F, t11951: F, t3211: F, t1025: F, t3218: F, t371: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42410, t42415, t42416, t42417, t42421, t42425, t42439) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2443::<F>(t11853, t828, t3229, t360, t3089, t1087, t11672, t11711, t1024, t12003, t11744, t3188);
        let (t42450, t42454, t42471, t42477, t42481) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2444::<F>(t3181, t675, t1063, t247, t2853, t11151, t11725, t283, t2852, t11951, t3211, t1025, t3218, t371, t676);
    (t42410, t42415, t42416, t42417, t42421, t42425, t42439, t42450, t42454, t42471, t42477, t42481)
}
