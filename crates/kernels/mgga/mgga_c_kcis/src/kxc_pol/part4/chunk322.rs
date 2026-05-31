//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 322/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk322<F: Float>(t1121: F, t388: F, t387: F, t1187: F, t279: F, t383: F, t280: F, t251: F) -> (F, F, F, F, F) {
    let t1188 = t388 * t1121;
    let t1189 = t387 * t1188;
    let t1190 = t1187 * t1189;
    let t1192 = t383 * t279;
    let t1194 = F::cast_from(1.0_f64) / t280 / t1192;
    let t1195 = t1194 * t251;
    (t1188, t1189, t1190, t1194, t1195)
}
