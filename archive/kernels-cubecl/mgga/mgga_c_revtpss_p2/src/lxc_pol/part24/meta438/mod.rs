//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1393;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta438<F: Float>(t1337: F, t40101: F, t1340: F, t40097: F, t39816: F, t1333: F, t9855: F, t19: F, t2237: F, t521: F, t9342: F, t14: F, t27: F, t583: F, t596: F, t525: F, t9603: F, t527: F, t9615: F, t40165: F, t268: F, t520: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46992, t46996, t46998, t47000, t47003, t47014, t47016) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1393::<F>(t1337, t40101, t1340, t40097, t39816, t1333, t9855, t19, t2237, t521, t9342, t14, t27);
        let (t47017, t47020, t47025, t47040, t47059, t47065) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1394::<F>(t47016, t521, t583, t596, t525, t9603, t527, t9615, t1340, t40165, t268, t520);
    (t46992, t46996, t46998, t47000, t47003, t47014, t47017, t47020, t47025, t47040, t47059, t47065)
}
