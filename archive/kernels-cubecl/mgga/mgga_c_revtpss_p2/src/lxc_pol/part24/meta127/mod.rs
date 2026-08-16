//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk671;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk672;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk673;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta127<F: Float>(t1592: F, t3109: F, t247: F, t1063: F, t1670: F, t3172: F, t1041: F, t1065: F, t1651: F, t1062: F, t1659: F, t3204: F, t127: F, t1663: F, t371: F, t1025: F, t225: F, t4746: F, t366: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4817, t4818, t4820, t4821, t4823, t4834) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk671::<F>(t1592, t3109, t247, t1063, t1670, t3172, t1041, t1065, t1651, t1062, t1659);
        let t4837 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk672::<F>(t1062, t3204);
        let (t4845, t4846, t4857) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk673::<F>(t127, t1663, t371, t1025, t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk674::<F>(t366, t4857);
    (t4817, t4818, t4820, t4821, t4823, t4834, t4837, t4845, t4846, t4857, t4858)
}
