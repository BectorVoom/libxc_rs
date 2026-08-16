//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1110/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1110<F: Float>(t36088: F, t36090: F, t41191: F, t41230: F, t41233: F, t41241: F, t41247: F, t43530: F, t46123: F, t46126: F, t46130: F, t46133: F, t46135: F, t46140: F, t46143: F, t46146: F) -> F {
    let t49064 = -F::cast_from(0.21168090156402149135e-3_f64) * t46123 + F::cast_from(0.13637330827122670865e-1_f64) * t46126 - F::cast_from(0.18183107769496894486e-1_f64) * t46130 - F::cast_from(0.39027158139407968655e0_f64) * t41191 - t43530 + F::cast_from(0.68186654135613354324e-2_f64) * t46133 - F::cast_from(0.90915538847484472432e-2_f64) * t46135 + F::cast_from(0.17740875559651324989e-2_f64) * t36088 - F::cast_from(0.2069768815292654582e-2_f64) * t36090 + F::cast_from(0.35403077613494883571e-2_f64) * t41230 - F::cast_from(0.42483693136193860285e-2_f64) * t41233 + F::cast_from(0.297385851953357022e-1_f64) * t41241 + F::cast_from(0.15577354149937748771e-1_f64) * t41247 - F::cast_from(0.2727466165424534173e-1_f64) * t46140 + F::cast_from(0.45457769423742236216e-1_f64) * t46143 + F::cast_from(0.3628815455383225566e-2_f64) * t46146;
    t49064
}
