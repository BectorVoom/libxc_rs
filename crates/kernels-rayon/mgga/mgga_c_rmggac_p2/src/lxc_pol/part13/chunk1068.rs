//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1068/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1068(t40121: f64, t35567: f64, t35577: f64, t35580: f64, t35584: f64, t35587: f64, t35591: f64, t35594: f64, t40123: f64, t40125: f64, t40136: f64, t40139: f64, t40143: f64, t40149: f64, t40154: f64, t40159: f64, t40164: f64) -> f64 {
    let t43288 = 0.11918087970123395032e-3_f64 * t40121;
    let t43302 = -0.11918087970123395032e-3_f64 * t35567 - 0.19863479950205658386e-4_f64 * t35577 - 0.39726959900411316772e-4_f64 * t35580 + t43288 + 0.49658699875514145966e-4_f64 * t40123 + 0.49658699875514145966e-4_f64 * t40125 - 0.11708147441822390596e1_f64 * t35584 + 0.17562221162733585894e1_f64 * t35587 + 0.5854073720911195298e0_f64 * t35591 - 0.20455996240684006298e-1_f64 * t40136 + 0.40911992481368012596e-1_f64 * t40139 - 0.212822999466489197e-4_f64 * t40143 - 0.425645998932978394e-4_f64 * t40149 - 0.3405167991463827152e-4_f64 * t40154 + 0.10215503974391481456e-3_f64 * t40159 - 0.10215503974391481456e-3_f64 * t40164 + 0.79828278012425390427e-1_f64 * t35594;
    t43302
}
