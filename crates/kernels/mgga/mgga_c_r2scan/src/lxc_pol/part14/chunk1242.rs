//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1242/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1242<F: Float>(t322: F, t41875: F, t41890: F, t12241: F, t833: F, t1299: F, t3730: F, t1013: F, t11220: F, t12244: F, t1292: F, t1295: F, t1300: F, t2394: F, t327: F, t3509: F, t38834: F, t6693: F, t829: F, t834: F, t8398: F) -> (F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t41891 = t41875 + t41890;
    let t41892 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t41891);
    let t41901 = t12241 * t833;
    let t41906 = t3730 * t1299;
    let t41917 = -F::cast_from(0.256e1_f64) * t1300 * t12241 * t829 - F::cast_from(0.64e0_f64) * t41892 * t327 - F::cast_from(0.128e1_f64) * t1300 * t3730 * t1292 - F::cast_from(0.384e1_f64) * t6693 * t3730 * t1295 - F::cast_from(0.256e1_f64) * t41901 * t829 - F::cast_from(0.128e1_f64) * t12244 * t1292 - F::cast_from(0.384e1_f64) * t41906 * t1295 - F::cast_from(0.128e1_f64) * t38834 * t1013 - F::cast_from(0.256e1_f64) * t11220 * t2394 - F::cast_from(0.128e1_f64) * t3509 * t8398 - F::cast_from(0.64e0_f64) * t834 * t41892;
    (t41891, t41917)
}
