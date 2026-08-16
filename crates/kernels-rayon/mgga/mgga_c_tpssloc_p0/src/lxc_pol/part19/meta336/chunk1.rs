//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1201/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201(t193: f64, t202: f64, t2522: f64, t39529: f64, t40760: f64, t40762: f64, t40764: f64, t40766: f64, t40768: f64, t40769: f64, t40772: f64, t40777: f64, t40779: f64, t40782: f64, t40784: f64, t40785: f64, t40790: f64, t776: f64) -> f64 {
    let t40791 = -6.0_f64 * t193 * t202 * t40769 * t40772 + 24.0_f64 * t2522 * t40785 * t776 - t39529 + t40760 - t40762 + t40764 + t40766 + t40768 + t40777 - t40779 + t40782 + t40784 + t40790;
    t40791
}
