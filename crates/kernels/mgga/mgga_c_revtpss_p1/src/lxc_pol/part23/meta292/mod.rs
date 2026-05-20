//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1525;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1526;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1527;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1528;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta292<F: Float>(t290: F, t2925: F, t2967: F, t941: F, t2966: F, t307: F, t302: F, t11132: F, t11337: F, t944: F, t2969: F, t310: F, t3011: F, t960: F, t3010: F, t320: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11387, t11404, t11408, t11409) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1525::<F>(t290, t2925, t2967, t941, t2966, t307, t302);
        let (t11422, t11423, t11449, t11450) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1526::<F>(t11132, t11337, t2966, t944, t302);
        let (t11452, t11461, t11465) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1527::<F>(t2969, t310, t3011, t960, t3010, t320);
        let t11466 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1528::<F>(t11465, t315);
    (t11387, t11404, t11408, t11409, t11422, t11423, t11449, t11450, t11452, t11461, t11465, t11466)
}
