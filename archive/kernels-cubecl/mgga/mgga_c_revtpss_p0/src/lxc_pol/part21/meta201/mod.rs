//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1208;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1209;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1210;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1211;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1212;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1213;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta201<F: Float>(t366: F, t4797: F, t1065: F, t2857: F, t4181: F, t1042: F, t2852: F, t3181: F, t1592: F, t3109: F, t247: F, t1063: F, t1670: F, t3172: F, t1041: F, t1651: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4798, t4801) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1208::<F>(t366, t4797, t1065, t2857);
        let (t4802, t4803) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1209::<F>(t4181, t4801, t1042);
        let t4806 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1210::<F>(t2852, t3181);
        let (t4807, t4808) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1211::<F>(t4181, t4806, t1042);
        let t4817 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1212::<F>(t1592, t3109, t247);
        let (t4818, t4820) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1213::<F>(t1063, t4817, t1670, t3172);
        let (t4821, t4823) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1214::<F>(t1041, t4820, t1065, t1651);
    (t4798, t4801, t4802, t4803, t4806, t4807, t4808, t4817, t4818, t4820, t4821, t4823)
}
