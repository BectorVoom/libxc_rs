//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1235;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1236;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1237;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1238;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1239;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta195<F: Float>(t4635: F, t934: F, t2924: F, t2848: F, t2930: F, t4571: F, t4576: F, t4581: F, t4585: F, t1614: F, t945: F, t1622: F, t953: F, t2906: F, t2950: F, t2957: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t4623: F, t4626: F, t4629: F, t954: F, t1621: F, t2970: F, t2974: F, t324: F, t1626: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4636, t4638, t4644, t4647) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1235::<F>(t4635, t934, t2924, t2848, t2930, t4571, t4576, t4581, t4585, t1614, t945);
        let (t4652, t4669) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1236::<F>(t1622, t953, t2848, t2906, t2950, t2957, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620, t4623, t4626, t4629);
        let t4670 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1237::<F>(t4669, t954);
        let t4673 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1238::<F>(t1621, t2970);
        let (t4674, t4682) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1239::<F>(t4673, t953, t2848, t2974, t4571, t4576, t4581, t4585);
        let (t4683, t4685) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1240::<F>(t324, t4682, t1626, t964);
    (t4636, t4638, t4644, t4647, t4652, t4669, t4670, t4673, t4674, t4682, t4683, t4685)
}
