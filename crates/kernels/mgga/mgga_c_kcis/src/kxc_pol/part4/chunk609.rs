//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 609/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk609<F: Float>(t3483: F, t414: F, t1242: F, t1247: F, t1241: F, t68: F) -> (F, F, F) {
    let t3484 = t414 * t3483;
    let t3487 = t1242 * t1247;
    let t3489 = t1241 * t68;
    let t3490 = t414 * t3489;
    (t3484, t3487, t3490)
}
