//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1357/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1357(t95903: f64, t26960: f64, t28102: f64, t7775: f64, t7796: f64, t8087: f64, t92830: f64, t93082: f64, t95895: f64, t95906: f64, t97010: f64, t97015: f64, t97019: f64, t97026: f64, t97028: f64, t97030: f64) -> f64 {
    let t97031 = 0.15476481481481481481e-2_f64 * t95903;
    let t97033 = -0.24734586805555555556e-3_f64 * t92830 * t8087 + 0.23214722222222222222e-2_f64 * t95895 - 0.18534722222222222222e-2_f64 * t97010 * t7796 - 0.18534722222222222222e-2_f64 * t97010 * t7775 - 0.24734586805555555556e-3_f64 * t97015 * t7775 - 0.23168402777777777778e-3_f64 * t26960 * t97019 - 0.82448622685185185185e-4_f64 * t93082 * t28102 - t97026 - t97028 - t97030 - t97031 + 0.61905925925925925925e-2_f64 * t95906;
    t97033
}
