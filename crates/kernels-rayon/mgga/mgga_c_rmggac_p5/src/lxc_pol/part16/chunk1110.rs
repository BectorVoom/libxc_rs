//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1110/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1110(t36088: f64, t36090: f64, t41191: f64, t41230: f64, t41233: f64, t41241: f64, t41247: f64, t43530: f64, t46123: f64, t46126: f64, t46130: f64, t46133: f64, t46135: f64, t46140: f64, t46143: f64, t46146: f64) -> f64 {
    let t49064 = -0.21168090156402149135e-3_f64 * t46123 + 0.13637330827122670865e-1_f64 * t46126 - 0.18183107769496894486e-1_f64 * t46130 - 0.39027158139407968655e0_f64 * t41191 - t43530 + 0.68186654135613354324e-2_f64 * t46133 - 0.90915538847484472432e-2_f64 * t46135 + 0.17740875559651324989e-2_f64 * t36088 - 0.2069768815292654582e-2_f64 * t36090 + 0.35403077613494883571e-2_f64 * t41230 - 0.42483693136193860285e-2_f64 * t41233 + 0.297385851953357022e-1_f64 * t41241 + 0.15577354149937748771e-1_f64 * t41247 - 0.2727466165424534173e-1_f64 * t46140 + 0.45457769423742236216e-1_f64 * t46143 + 0.3628815455383225566e-2_f64 * t46146;
    t49064
}
