//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1236/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1236<F: Float>(t14160: F, t40574: F, t43744: F, t11531: F, t11629: F, t3275: F, t11498: F, t40282: F, t11502: F, t40664: F, t11556: F, t40713: F) -> (F, F, F, F, F) {
    let t43747 = F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t40574 * t14160 * t43744;
    let t43750 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t11629 * t11531;
    let t43752 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40282 * t11498;
    let t43754 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40664 * t11502;
    let t43756 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t40713 * t11556;
    (t43747, t43750, t43752, t43754, t43756)
}
