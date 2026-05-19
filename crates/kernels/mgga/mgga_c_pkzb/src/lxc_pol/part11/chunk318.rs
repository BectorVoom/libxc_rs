//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 318/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk318<F: Float>(t1072: F, t1086: F, t1088: F, t1096: F, t1101: F, t1108: F, t237: F, t248: F, t695: F, t714: F, t1107: F, t713: F, t722: F) -> (F, F, F) {
    let t1112 = t237 * (-F::new(0.310907e-1) * t1088 * t248 + F::new(1.0) * t695 * t1096 + t1072 - t1086 - F::cast_from(0.19751673498613801407e-1_f64) * t1101 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t1108);
    let t1114 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t1101;
    let t1116 = t713 * t1107 * t722;
    (t1112, t1114, t1116)
}
