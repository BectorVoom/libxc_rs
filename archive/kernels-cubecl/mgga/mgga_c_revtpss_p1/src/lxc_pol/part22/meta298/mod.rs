//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1726;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1727;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta298<F: Float>(t4003: F, t9768: F, t9934: F, t2661: F, t532: F, t549: F, t240: F, t72: F, t595: F, t66: F, t247: F, t550: F, t548: F, t4010: F, t245: F, t3829: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9935, t9936, t9937, t9940, t9941, t9942, t9948) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1726::<F>(t4003, t9768, t9934, t2661, t532, t549, t240, t72, t595, t66);
        let (t9949, t9953, t9954, t9955) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1727::<F>(t240, t9948, t247, t550, t548, t4010, t72, t245);
        let t9956 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1728::<F>(t3829, t543);
    (t9935, t9936, t9937, t9940, t9941, t9942, t9948, t9949, t9953, t9954, t9955, t9956)
}
