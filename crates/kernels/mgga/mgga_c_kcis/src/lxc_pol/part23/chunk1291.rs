//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1291/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1291<F: Float>(t12825: F, t7978: F, t8221: F, t27591: F, t28727: F, t28714: F, t2104: F, t27614: F, t4468: F, t6176: F, t27556: F, t27595: F, t27617: F, t27638: F, t28738: F, t7968: F, t94914: F, t94916: F, t99074: F, t99079: F) -> (F, F) {
    let t99152 = t7978 * t12825 * t8221;
    let t99154 = t28727 * t27591;
    let t99157 = F::cast_from(0.7722800925925925926e-4_f64) * t28714 * t27591;
    let t99166 = t6176 * t27614 * t2104 * t4468;
    let t99169 = F::cast_from(0.18534722222222222222e-2_f64) * t28727 * t27638 + F::cast_from(0.11584201388888888889e-3_f64) * t94914 + F::cast_from(0.30918233506944444444e-4_f64) * t94916 - F::cast_from(0.69505208333333333334e-3_f64) * t28714 * t27617 + F::cast_from(0.25742669753086419753e-4_f64) * t99152 + F::cast_from(0.20594135802469135802e-3_f64) * t99154 - t99157 - F::cast_from(0.13913205078125e-3_f64) * t7968 * t99074 + F::cast_from(0.557015165302734375e-4_f64) * t27595 * t99079 - F::cast_from(0.92754700520833333334e-4_f64) * t27556 * t28738 - F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t99166;
    (t99166, t99169)
}
