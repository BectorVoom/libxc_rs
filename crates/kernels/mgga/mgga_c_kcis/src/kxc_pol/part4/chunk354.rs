//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 354/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk354<F: Float>(t1334: F, t1335: F, t1316: F, t1305: F, t1309: F) -> (F, F, F, F) {
    let t1336 = t1334 * t1335;
    let t1338 = F::new(1.0) * t1316 * t1336;
    let t1339 = F::cast_from(0.92708333333333333333e-2_f64) * t1305;
    let t1341 = -t1339 - F::cast_from(0.92708333333333333333e-2_f64) * t1309;
    (t1336, t1338, t1339, t1341)
}
