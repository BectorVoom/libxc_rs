//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1252/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1252<F: Float>(t1628: F, t7533: F, t1636: F, t7537: F, t2128: F, t6256: F, t7566: F, t22349: F, t22352: F, t22355: F, t22359: F, t22362: F, t22365: F, t22367: F, t22369: F, t22371: F, t22374: F, t22377: F) -> (F, F, F, F, F) {
    let t23255 = t7533 * t1628;
    let t23265 = t7537 * t1636;
    let t23268 = t2128 * t6256;
    let t23272 = t7566 * t1636;
    let t23297 = -0.20234375e-1 * t22349 + 0.375e0 * t22352 + 0.89930555555555555553e-2 * t22355 - 0.9375e-1 * t22359 + 0.1875e0 * t22362 + 0.13489583333333333333e-1 * t22365 - 0.14388888888888888889e0 * t22367 - 0.1875e0 * t22369 - 0.14388888888888888889e0 * t22371 + 0.125e0 * t22374 + 0.27777777777777777777e-1 * t22377;
    (t23255, t23265, t23268, t23272, t23297)
}
