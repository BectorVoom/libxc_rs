//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta198 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1227;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1228;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1229;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1230;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1231;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta198<F: Float>(t1117: F, t4785: F, t3313: F, t3238: F, t3319: F, t4721: F, t4726: F, t4731: F, t4735: F, t1128: F, t1675: F, t1136: F, t1683: F, t3295: F, t3339: F, t3346: F, t4749: F, t4757: F, t4765: F, t4767: F, t4770: F, t4773: F, t4776: F, t4779: F, t1137: F, t1682: F, t3359: F, t3363: F, t449: F, t1147: F, t1687: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4786, t4788, t4794, t4797) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1227::<F>(t1117, t4785, t3313, t3238, t3319, t4721, t4726, t4731, t4735, t1128, t1675);
        let (t4802, t4819) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1228::<F>(t1136, t1683, t3238, t3295, t3339, t3346, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770, t4773, t4776, t4779);
        let t4820 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1229::<F>(t1137, t4819);
        let t4823 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1230::<F>(t1682, t3359);
        let (t4824, t4832) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1231::<F>(t1136, t4823, t3238, t3363, t4721, t4726, t4731, t4735);
        let (t4833, t4835) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1232::<F>(t449, t4832, t1147, t1687);
    (t4786, t4788, t4794, t4797, t4802, t4819, t4820, t4823, t4824, t4832, t4833, t4835)
}
