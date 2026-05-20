//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1818;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1819;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1820;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1821;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta343<F: Float>(t11132: F, t11337: F, t2966: F, t944: F, t302: F, t2969: F, t310: F, t2979: F, t964: F, t3011: F, t960: F, t3010: F, t320: F, t315: F, t963: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11422, t11423, t11449, t11450) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1818::<F>(t11132, t11337, t2966, t944, t302);
        let (t11452, t11456, t11461) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1819::<F>(t2969, t310, t2979, t964, t3011, t960);
        let t11465 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1820::<F>(t3010, t320);
        let t11466 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1821::<F>(t11465, t315);
        let (t11479, t11480, t11506) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1822::<F>(t11132, t11337, t3010, t963);
    (t11422, t11423, t11449, t11450, t11452, t11456, t11461, t11465, t11466, t11479, t11480, t11506)
}
