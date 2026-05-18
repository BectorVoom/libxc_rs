//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 792/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk792<F: Float>(t12216: F, t9: F, t1362: F, t486: F, t3716: F, t503: F) -> (F, F, F, F, F) {
    let t12217 = t9 * t12216;
    let t12229 = t1362 * t1362;
    let t12230 = F::new(1.0) / t12229;
    let t12231 = t486 * t12230;
    let t12234 = F::new(1.0) / t3716 / t503;
    (t12217, t12229, t12230, t12231, t12234)
}
