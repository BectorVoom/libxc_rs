//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1413/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1413(t43819: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64) -> f64 {
    let t43895 = 0.31310740740740740741e1_f64 * t43819;
    let t43909 = t43895 + 0.80513333333333333336e0_f64 * t43780 + 0.16102666666666666667e1_f64 * t43782 + 0.16102666666666666667e1_f64 * t43784 - 0.24154e1_f64 * t43786 - 0.40256666666666666668e0_f64 * t43788 + 0.40256666666666666666e1_f64 * t43794 - 0.72462e1_f64 * t43798 + 0.72462e1_f64 * t43802 + 0.301925e0_f64 * t43806 - 0.89459259259259259259e0_f64 * t43811 - 0.12524296296296296297e1_f64 * t43816 - 0.60384999999999999999e0_f64 * t43823 + 0.181155e1_f64 * t43828;
    t43909
}
