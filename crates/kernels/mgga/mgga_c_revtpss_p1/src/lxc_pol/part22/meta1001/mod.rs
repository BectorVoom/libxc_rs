//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1001 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3406;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3407;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1001<F: Float>(t4707: F, t3011: F, t3014: F, t981: F, t11108: F, t6396: F, t2874: F, t63657: F, t935: F, t19471: F, t3022: F, t15534: F, t4719: F, t19133: F, t2989: F, t15559: F, t11591: F, t6223: F, t19049: F, t3026: F, t15556: F, t19146: F, t3007: F, t16612: F, t19137: F, t3329: F, t3333: F, t5023: F, t5024: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63902, t63906, t63907, t63916, t63918, t63920) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3406::<F>(t4707, t3011, t3014, t981, t11108, t6396, t2874, t63657, t935, t19471, t3022, t15534, t4719);
        let (t63923, t63925, t63927, t63929, t63934, t63937) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3407::<F>(t19133, t2989, t981, t15559, t4719, t11591, t6223, t19049, t3026, t15556, t19146, t3007);
        let t63938 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3408::<F>(t16612, t19137, t3329, t3333, t5023, t5024, t63906, t63907, t63916, t63918, t63920, t63923, t63925, t63927, t63929, t63934, t63937);
    (t63902, t63906, t63916, t63918, t63920, t63923, t63925, t63927, t63929, t63934, t63937, t63938)
}
