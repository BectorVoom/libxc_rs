//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1057;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta212(t1100: f64, t4756: f64, t1661: f64, t3287: f64, t1102: f64, t1107: f64, t1667: f64, t699: f64, t3297: f64, t4724: f64, t136: f64, t1113: f64, t4729: f64, t4733: f64, t3238: f64, t3282: f64, t3294: f64, t3295: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t4749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4757, t4764, t4765, t4767, t4770, t4772, t4773, t4775) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1057(t1100, t4756, t1661, t3287, t1102, t1107, t1667, t699, t3297, t4724, t136, t1113, t4729);
        let (t4776, t4778, t4779, t4781) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1058(t136, t4775, t1113, t4733, t3238, t3282, t3294, t3295, t4721, t4726, t4731, t4735, t4749, t4757, t4765, t4767, t4770, t4773);
    (t4757, t4764, t4765, t4767, t4770, t4772, t4773, t4775, t4776, t4778, t4779, t4781)
}
