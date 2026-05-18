//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1098/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1098<F: Float>(t1356: F, t37419: F, t40332: F, t43366: F, t43385: F, t45556: F, t46933: F, t46938: F, t46943: F, t46948: F, t46953: F, t46958: F, t46963: F, t46969: F, t46974: F, t46977: F, t46981: F, t46985: F, t46989: F) -> F {
    let t48838 = F::new(0.47896966807455234256e0) * t1356 * t37419 * t45556 - F::new(0.23942587439980034662e-4) * t46933 + F::new(0.71827762319940103986e-4) * t46938 - F::new(0.71827762319940103986e-4) * t46943 - F::new(0.23942587439980034662e-4) * t46948 - t43366 - F::new(0.638468998399467591e-4) * t46953 - F::new(0.212822999466489197e-4) * t46958 + F::new(0.212822999466489197e-4) * t46963 - F::new(0.1702583995731913576e-4) * t46969 + F::new(0.638468998399467591e-4) * t46974 - F::new(0.11708147441822390596e1) * t40332 - F::new(0.68186654135613354325e-2) * t46977 - F::new(0.36366215538993788973e-1) * t46981 - F::new(0.2553875993597870364e-4) * t46985 + t43385 - F::new(0.2553875993597870364e-4) * t46989;
    t48838
}
