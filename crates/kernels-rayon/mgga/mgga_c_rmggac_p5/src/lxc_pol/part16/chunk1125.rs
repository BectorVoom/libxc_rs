//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1125/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1125(t40944: f64, t40949: f64, t40951: f64, t40966: f64, t44093: f64, t44095: f64, t46327: f64, t46329: f64, t46331: f64, t46343: f64, t46346: f64, t46349: f64) -> f64 {
    let t49323 = 0.11708147441822390596e1_f64 * t40944 - 0.17562221162733585894e1_f64 * t40949 - 0.58540737209111952978e0_f64 * t40951 - 0.40911992481368012595e0_f64 * t46327 + 0.8182398496273602519e0_f64 * t46329 + 0.13637330827122670865e0_f64 * t46331 - 0.16364796992547205038e0_f64 * t46343 + 0.2727466165424534173e0_f64 * t46346 + 0.10909864661698136692e0_f64 * t46349 + 0.72732431077987577948e-1_f64 * t40966 - t44093 + t44095;
    t49323
}
