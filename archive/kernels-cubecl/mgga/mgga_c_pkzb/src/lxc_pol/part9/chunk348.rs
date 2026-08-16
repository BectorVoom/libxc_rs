//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 348/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk348<F: Float>(t1173: F, t1187: F, t1189: F, t1197: F, t1202: F, t1209: F, t237: F, t365: F, t863: F, t882: F, t1208: F, t881: F, t890: F) -> (F, F, F) {
    let t1213 = t237 * (-F::cast_from(0.310907e-1_f64) * t1189 * t365 + F::cast_from(1.0_f64) * t863 * t1197 + t1173 - t1187 - F::cast_from(0.19751673498613801407e-1_f64) * t1202 + F::cast_from(0.5848223622634646207e0_f64) * t882 * t1209);
    let t1215 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t1202;
    let t1217 = t881 * t1208 * t890;
    (t1213, t1215, t1217)
}
