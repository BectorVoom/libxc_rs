//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 705/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk705<F: Float>(t1166: F, t4869: F, t1703: F, t3411: F, t1694: F, t3375: F, t1157: F, t1164: F, t1147: F, t1156: F, t4857: F, t3400: F) -> (F, F, F, F, F, F, F, F) {
    let t4871 = F::cast_from(0.5848223622634646207e0_f64) * t4869 * t1166;
    let t4873 = F::cast_from(0.5848223622634646207e0_f64) * t3411 * t1703;
    let t4874 = t3375 * t1694;
    let t4875 = t4874 * t1157;
    let t4877 = F::cast_from(0.11696447245269292414e1_f64) * t1164 * t4875;
    let t4879 = t1147 * t4857 * t1156;
    let t4881 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t4879;
    let t4882 = t3400 * t1694;
    (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882)
}
