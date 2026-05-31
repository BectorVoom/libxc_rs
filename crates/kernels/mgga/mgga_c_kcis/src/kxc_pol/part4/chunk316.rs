//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 316/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk316<F: Float>(t1154: F, t1155: F, t829: F, t1083: F, t304: F, t1110: F, t1115: F, t1143: F, t1152: F, t1153: F, t348: F, t365: F, t368: F, t86: F) -> (F, F, F) {
    let t1157 = t1154 * t1155 * t829;
    let t1160 = t304 * t1083;
    let t1164 = F::cast_from(0.619125e-2_f64) * t1143 * t348 + F::cast_from(0.9286875e-2_f64) * t365 * t1110 - F::cast_from(0.619125e-2_f64) * t365 * t1115 - t1152 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t1157 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t1160;
    (t1157, t1160, t1164)
}
