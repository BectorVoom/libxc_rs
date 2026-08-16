//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1227/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1227(t39529: f64, t40755: f64, t40760: f64, t40762: f64, t40764: f64, t40766: f64, t40768: f64, t40777: f64, t40779: f64, t40782: f64, t40784: f64, t39549: f64, t40790: f64, t40793: f64, t40795: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t40805: f64, t40807: f64, t40809: f64, t40811: f64) -> (f64, f64) {
    let t41248 = t40755 + t40760 - t40762 + t40764 + t40766 + t40768 + t40777 - t39529 - t40779 + t40782 + t40784;
    let t41249 = t40790 + t40793 + t40795 + t40797 + t40799 + t40801 - t40803 - t40805 + t40807 + t40809 + t40811 + t39549;
    (t41248, t41249)
}
