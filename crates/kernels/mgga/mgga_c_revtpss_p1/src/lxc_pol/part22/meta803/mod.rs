//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta803 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2904;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta803<F: Float>(t4038: F, t9323: F, t1340: F, t40097: F, t39816: F, t1333: F, t9855: F, t19: F, t2237: F, t521: F, t1331: F, t9342: F, t2619: F, t9563: F, t3825: F, t9586: F, t14: F, t27: F, t525: F, t9603: F, t527: F, t9615: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46993, t46996, t46998, t46999, t47003, t47005) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2904::<F>(t4038, t9323, t1340, t40097, t39816, t1333, t9855, t19, t2237, t521, t1331, t9342);
        let (t47007, t47009, t47011, t47016, t47025, t47040) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2905::<F>(t1331, t9855, t2619, t9563, t3825, t9586, t14, t27, t521, t525, t9603, t527, t9615);
    (t46993, t46996, t46998, t46999, t47003, t47005, t47007, t47009, t47011, t47016, t47025, t47040)
}
