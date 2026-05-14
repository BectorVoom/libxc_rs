//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 345/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk345<F: Float>(t1319: F, t1324: F, t250: F, t324: F, t461: F, t251: F, t494: F) -> (F, F, F, F) {
    let t1325 = t1324 * t1319;
    let t1328 = t250 * t324 * t461;
    let t1329 = 0.82156666666666666667e-1 * t1328;
    let t1330 = t251 * t494;
    (t1325, t1328, t1329, t1330)
}
