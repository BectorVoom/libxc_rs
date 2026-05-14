//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1050/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1050<F: Float>(t7258: F, t7931: F, t303: F, t27331: F, t6928: F, t6932: F, t1598: F, t21868: F, t1982: F, t8175: F, t552: F, t556: F, t7052: F, t541: F, t1364: F, t23157: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29380 = t7931 * t7258;
    let t29381 = t303 * t29380;
    let t29383 = t27331 * t6928;
    let t29384 = t303 * t29383;
    let t29386 = t7931 * t6932;
    let t29387 = t303 * t29386;
    let t29393 = t21868 * t1598;
    let t29396 = t1982 * t8175;
    let t29397 = t303 * t29396;
    let t29400 = t552 * t7052 * t556;
    let t29401 = t541 * t29400;
    let t29402 = t303 * t29401;
    let t29404 = t1364 * t23157;
    (t29380, t29381, t29383, t29384, t29386, t29387, t29393, t29396, t29397, t29400, t29401, t29402, t29404)
}
