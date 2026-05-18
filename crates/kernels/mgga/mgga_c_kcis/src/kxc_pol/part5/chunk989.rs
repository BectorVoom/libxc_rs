//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 989/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk989<F: Float>(t1376: F, t1370: F, t3999: F, t1377: F, t3978: F, t1444: F, t451: F, t9: F, t1362: F, t486: F, t3716: F, t503: F) -> (F, F, F, F, F, F) {
    let t12158 = t1376 * t1376;
    let t12159 = F::new(1.0) / t12158;
    let t12185 = t1370 * t3999;
    let t12194 = t3978 * t1377;
    let t12216 = F::new(1.0) / t451 / t1444;
    let t12217 = t9 * t12216;
    let t12229 = t1362 * t1362;
    let t12230 = F::new(1.0) / t12229;
    let t12231 = t486 * t12230;
    let t12234 = F::new(1.0) / t3716 / t503;
    (t12159, t12185, t12194, t12217, t12231, t12234)
}
