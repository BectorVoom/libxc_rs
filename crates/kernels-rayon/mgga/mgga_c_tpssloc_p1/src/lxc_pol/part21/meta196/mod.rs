//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1222;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta196(t1119: f64, t4740: f64, t1671: f64, t3259: f64, t1117: f64, t3264: f64, t1661: f64, t3270: f64, t1102: f64, t3238: f64, t3274: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t1100: f64, t3287: f64, t1107: f64, t1667: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4742, t4744, t4745, t4747, t4748, t4749, t4756) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1222(t1119, t4740, t1671, t3259, t1117, t3264, t1661, t3270, t1102, t3238, t3274, t4721, t4726, t4731, t4735);
        let (t4757, t4764, t4765, t4767, t4770) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1223(t1100, t4756, t1661, t3287, t1102, t1107, t1667, t699);
    (t4742, t4744, t4745, t4747, t4748, t4749, t4756, t4757, t4764, t4765, t4767, t4770)
}
