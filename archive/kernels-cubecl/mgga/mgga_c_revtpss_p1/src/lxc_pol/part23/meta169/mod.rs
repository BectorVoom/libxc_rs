//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1016;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1017;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1018;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta169<F: Float>(t4181: F, t4801: F, t1042: F, t2852: F, t3181: F, t1592: F, t3109: F, t247: F, t1063: F, t1670: F, t3172: F, t1041: F, t1065: F, t1651: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4802, t4803, t4806) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1016::<F>(t4181, t4801, t1042, t2852, t3181);
        let (t4807, t4808, t4817) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1017::<F>(t4181, t4806, t1042, t1592, t3109, t247);
        let (t4818, t4820) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1018::<F>(t1063, t4817, t1670, t3172);
        let (t4821, t4823) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1019::<F>(t1041, t4820, t1065, t1651);
    (t4802, t4803, t4806, t4807, t4808, t4817, t4818, t4820, t4821, t4823)
}
