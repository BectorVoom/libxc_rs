//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk984;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta159<F: Float>(t1119: F, t4740: F, t1671: F, t3259: F, t1117: F, t3264: F, t1661: F, t3270: F, t1102: F, t3238: F, t3274: F, t4721: F, t4726: F, t4731: F, t4735: F, t1100: F, t3287: F, t1107: F, t1667: F, t699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4742, t4744, t4745, t4747, t4748, t4749, t4756) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk984::<F>(t1119, t4740, t1671, t3259, t1117, t3264, t1661, t3270, t1102, t3238, t3274, t4721, t4726, t4731, t4735);
        let (t4757, t4764, t4765, t4767, t4770) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk985::<F>(t1100, t4756, t1661, t3287, t1102, t1107, t1667, t699);
    (t4742, t4744, t4745, t4747, t4748, t4749, t4756, t4757, t4764, t4765, t4767, t4770)
}
