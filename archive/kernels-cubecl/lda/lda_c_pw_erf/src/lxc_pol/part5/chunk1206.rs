//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1206/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1206<F: Float>(t1953: F, t21815: F, t3633: F, t1333: F, t7370: F, t352: F, t11: F, t557: F, t1351: F, t1349: F, t16292: F, t16297: F, t16325: F, t16327: F, t16338: F, t21779: F, t21782: F, t21785: F, t21788: F, t21790: F, t21792: F, t21796: F, t21799: F, t21802: F, t21805: F, t21808: F, t21813: F) -> (F, F, F, F, F, F) {
    let t21817 = t1953 * t3633 * t21815;
    let t21819 = t1333 * t7370;
    let t21820 = t21819 * t352;
    let t21822 = t11 * t557 * t21820;
    let t21824 = t1351 * t7370;
    let t21825 = t21824 * t352;
    let t21827 = t11 * t1349 * t21825;
    let t21834 = -F::cast_from(0.04534_f64) * t21779 - F::cast_from(0.06801_f64) * t21782 - F::cast_from(0.011335_f64) * t21785 + F::cast_from(0.02267_f64) * t21788 - F::cast_from(0.0012594444444444445_f64) * t21790 + F::cast_from(0.003778333333333333_f64) * t21792 + F::cast_from(0.04534_f64) * t21796 - F::cast_from(0.02518888888888889_f64) * t21799 + F::cast_from(0.04534_f64) * t21802 + F::cast_from(0.003778333333333333_f64) * t21805 - F::cast_from(0.007556666666666666_f64) * t21808 + F::cast_from(0.005597530864197531_f64) * t21813 - F::cast_from(0.012594444444444445_f64) * t21817 - F::cast_from(0.003778333333333333_f64) * t21822 + F::cast_from(0.0012594444444444445_f64) * t21827 + F::cast_from(0.007556666666666666_f64) * t16292 - F::cast_from(0.002099074074074074_f64) * t16297 + F::cast_from(0.003778333333333333_f64) * t16325 - F::cast_from(0.0012594444444444445_f64) * t16327 + F::cast_from(0.005037777777777778_f64) * t16338;
    (t21817, t21820, t21822, t21825, t21827, t21834)
}
