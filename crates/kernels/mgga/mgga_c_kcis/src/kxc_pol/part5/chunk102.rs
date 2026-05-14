//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 102/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk102<F: Float>(t242: F, t245: F) -> (F, F, F, F) {
    let t308 = 0.107924e1 + 0.3964e-1 * t245 + 0.123825e-1 * t242;
    let t311 = 1.0 + t245 * t308 / 2.0;
    let t312 = t311 * t311;
    let t313 = 1.0 / t312;
    (t308, t311, t312, t313)
}
