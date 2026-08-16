//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1098/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1098(t1356: f64, t37419: f64, t40332: f64, t43366: f64, t43385: f64, t45556: f64, t46933: f64, t46938: f64, t46943: f64, t46948: f64, t46953: f64, t46958: f64, t46963: f64, t46969: f64, t46974: f64, t46977: f64, t46981: f64, t46985: f64, t46989: f64) -> f64 {
    let t48838 = 0.47896966807455234256e0_f64 * t1356 * t37419 * t45556 - 0.23942587439980034662e-4_f64 * t46933 + 0.71827762319940103986e-4_f64 * t46938 - 0.71827762319940103986e-4_f64 * t46943 - 0.23942587439980034662e-4_f64 * t46948 - t43366 - 0.638468998399467591e-4_f64 * t46953 - 0.212822999466489197e-4_f64 * t46958 + 0.212822999466489197e-4_f64 * t46963 - 0.1702583995731913576e-4_f64 * t46969 + 0.638468998399467591e-4_f64 * t46974 - 0.11708147441822390596e1_f64 * t40332 - 0.68186654135613354325e-2_f64 * t46977 - 0.36366215538993788973e-1_f64 * t46981 - 0.2553875993597870364e-4_f64 * t46985 + t43385 - 0.2553875993597870364e-4_f64 * t46989;
    t48838
}
