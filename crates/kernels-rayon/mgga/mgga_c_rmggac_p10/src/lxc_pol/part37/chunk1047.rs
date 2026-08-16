//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1047/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1047(t70819: f64, t74060: f64, t74069: f64, t74072: f64, t74075: f64, t74078: f64, t76842: f64, t76843: f64, t76844: f64, t76846: f64, t76848: f64, t76849: f64, t76854: f64, t76855: f64, t76856: f64, t76857: f64, t76858: f64) -> f64 {
    let t80034 = t76842 + t76843 - t76844 - t76846 + t70819 + 0.17451485956252114153e-4_f64 * t74060 + t76848 - t76849 + 0.17519306092901367186e-5_f64 * t74069 + 0.52557918278704101561e-6_f64 * t74072 - 0.52557918278704101561e-6_f64 * t74075 - 0.35038612185802734374e-6_f64 * t74078 - t76854 + t76855 + t76856 - t76857 - t76858;
    t80034
}
