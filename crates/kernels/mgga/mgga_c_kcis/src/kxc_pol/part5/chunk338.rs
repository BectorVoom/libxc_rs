//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 338/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk338<F: Float>(t1267: F, t1268: F, t1015: F, t1025: F, t1089: F, t1124: F, t1128: F, t1136: F, t1236: F, t1240: F, t430: F, t436: F) -> (F, F, F, F) {
    let t1269 = t1267 * t1268;
    let t1272 = F::cast_from(0.11607361111111111111e-2_f64) * t1015;
    let t1278 = t1236 * t430 - F::cast_from(0.66725e-1_f64) * t1240 * t1269 + t1272 + F::cast_from(0.11607361111111111111e-2_f64) * t1025 + F::cast_from(0.17411041666666666666e-2_f64) * t1089 - F::cast_from(0.17411041666666666666e-2_f64) * t1124 - F::cast_from(0.46429444444444444443e-2_f64) * t1128 + F::cast_from(0.11607361111111111111e-2_f64) * t1136;
    let t1280 = t436 * t436;
    (t1269, t1272, t1278, t1280)
}
