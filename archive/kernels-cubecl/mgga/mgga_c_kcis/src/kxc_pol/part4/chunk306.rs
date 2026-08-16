//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 306/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk306<F: Float>(t1022: F, t1121: F, t1096: F, t1092: F, t359: F, t983: F, t356: F, t303: F, t358: F) -> (F, F, F, F, F, F, F) {
    let t1122 = t1022 * t1121;
    let t1123 = t1096 * t1122;
    let t1124 = t1092 * t1123;
    let t1126 = t983 * t359;
    let t1127 = t356 * t1126;
    let t1128 = t303 * t1127;
    let t1130 = F::cast_from(1.0_f64) / t358;
    (t1122, t1123, t1124, t1126, t1127, t1128, t1130)
}
