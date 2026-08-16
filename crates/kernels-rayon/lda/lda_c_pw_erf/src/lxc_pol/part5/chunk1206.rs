//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1206/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1206(t1953: f64, t21815: f64, t3633: f64, t1333: f64, t7370: f64, t352: f64, t11: f64, t557: f64, t1351: f64, t1349: f64, t16292: f64, t16297: f64, t16325: f64, t16327: f64, t16338: f64, t21779: f64, t21782: f64, t21785: f64, t21788: f64, t21790: f64, t21792: f64, t21796: f64, t21799: f64, t21802: f64, t21805: f64, t21808: f64, t21813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21817 = t1953 * t3633 * t21815;
    let t21819 = t1333 * t7370;
    let t21820 = t21819 * t352;
    let t21822 = t11 * t557 * t21820;
    let t21824 = t1351 * t7370;
    let t21825 = t21824 * t352;
    let t21827 = t11 * t1349 * t21825;
    let t21834 = -0.04534_f64 * t21779 - 0.06801_f64 * t21782 - 0.011335_f64 * t21785 + 0.02267_f64 * t21788 - 0.0012594444444444445_f64 * t21790 + 0.003778333333333333_f64 * t21792 + 0.04534_f64 * t21796 - 0.02518888888888889_f64 * t21799 + 0.04534_f64 * t21802 + 0.003778333333333333_f64 * t21805 - 0.007556666666666666_f64 * t21808 + 0.005597530864197531_f64 * t21813 - 0.012594444444444445_f64 * t21817 - 0.003778333333333333_f64 * t21822 + 0.0012594444444444445_f64 * t21827 + 0.007556666666666666_f64 * t16292 - 0.002099074074074074_f64 * t16297 + 0.003778333333333333_f64 * t16325 - 0.0012594444444444445_f64 * t16327 + 0.005037777777777778_f64 * t16338;
    (t21817, t21820, t21822, t21825, t21827, t21834)
}
