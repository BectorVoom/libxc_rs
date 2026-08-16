//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk861;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk862;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta159<F: Float>(t342: F, t4980: F, t3154: F, t3302: F, t3316: F, t378: F, t1043: F, t357: F, t198: F, t336: F, t1187: F, t3523: F, t1263: F, t3367: F) -> (F, F, F, F, F, F, F, F) {
        let (t4981, t4982, t4995) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk861::<F>(t342, t4980, t3154, t3302, t3316, t378);
        let (t4996, t4998, t5023) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk862::<F>(t342, t4995, t1043, t3302, t357, t198, t336);
        let (t5206, t5268) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk863::<F>(t1187, t3523, t1263, t3367);
    (t4981, t4982, t4995, t4996, t4998, t5023, t5206, t5268)
}
