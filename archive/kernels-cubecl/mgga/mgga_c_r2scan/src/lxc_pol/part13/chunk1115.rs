//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1115/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1115<F: Float>(t3308: F, t574: F, t7940: F, t11797: F, t1584: F, t10776: F, t7442: F, t10772: F, t7449: F, t10698: F, t2559: F, t3295: F, t7934: F) -> (F, F, F, F, F, F) {
    let t39385 = t574 * t3308 * t7940;
    let t39387 = t1584 * t11797;
    let t39390 = t10776 * t3308 * t7442;
    let t39393 = t10772 * t3308 * t7449;
    let t39395 = t10698 * t2559;
    let t39396 = F::cast_from(0.12805040077930161442e0_f64) * t39395;
    let t39397 = t3295 * t7934;
    (t39385, t39387, t39390, t39393, t39396, t39397)
}
