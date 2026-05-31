//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 988/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk988<F: Float>(t499: F, t737: F, t493: F, t110: F, t1381: F, t109: F, t1369: F, t1372: F, t1368: F, t24: F, t3977: F, t1377: F, t3970: F) -> (F, F, F, F, F, F) {
    let t12127 = t737 * t499;
    let t12129 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t493 * t12127;
    let t12130 = t110 * t1381;
    let t12131 = t493 * t12130;
    let t12133 = t109 * t1369;
    let t12134 = t12133 * t1372;
    let t12135 = t1368 * t12134;
    let t12140 = t24 * t3977;
    let t12147 = t3970 * t1377;
    (t12129, t12131, t12133, t12135, t12140, t12147)
}
