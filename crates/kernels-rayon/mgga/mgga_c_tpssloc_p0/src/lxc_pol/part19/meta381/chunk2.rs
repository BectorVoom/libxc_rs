//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1425/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1425(t43819: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64) -> f64 {
    let t44053 = 0.31003950617283950618e1_f64 * t43819;
    let t44067 = t44053 + 0.79724444444444444446e0_f64 * t43780 + 0.15944888888888888889e1_f64 * t43782 + 0.15944888888888888889e1_f64 * t43784 - 0.23917333333333333333e1_f64 * t43786 - 0.39862222222222222223e0_f64 * t43788 + 0.39862222222222222223e1_f64 * t43794 - 0.71752000000000000002e1_f64 * t43798 + 0.71752e1_f64 * t43802 + 0.29896666666666666667e0_f64 * t43806 - 0.88582716049382716048e0_f64 * t43811 - 0.12401580246913580247e1_f64 * t43816 - 0.59793333333333333333e0_f64 * t43823 + 0.17938e1_f64 * t43828;
    t44067
}
