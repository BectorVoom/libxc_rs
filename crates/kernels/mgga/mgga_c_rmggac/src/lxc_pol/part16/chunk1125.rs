//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1125/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1125<F: Float>(t40944: F, t40949: F, t40951: F, t40966: F, t44093: F, t44095: F, t46327: F, t46329: F, t46331: F, t46343: F, t46346: F, t46349: F) -> F {
    let t49323 = F::cast_from(0.11708147441822390596e1_f64) * t40944 - F::cast_from(0.17562221162733585894e1_f64) * t40949 - F::cast_from(0.58540737209111952978e0_f64) * t40951 - F::cast_from(0.40911992481368012595e0_f64) * t46327 + F::cast_from(0.8182398496273602519e0_f64) * t46329 + F::cast_from(0.13637330827122670865e0_f64) * t46331 - F::cast_from(0.16364796992547205038e0_f64) * t46343 + F::cast_from(0.2727466165424534173e0_f64) * t46346 + F::cast_from(0.10909864661698136692e0_f64) * t46349 + F::cast_from(0.72732431077987577948e-1_f64) * t40966 - t44093 + t44095;
    t49323
}
