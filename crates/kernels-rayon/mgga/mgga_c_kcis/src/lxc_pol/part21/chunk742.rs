//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 742/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk742(t2197: f64, t7772: f64, t7786: f64, t7788: f64, t7801: f64, t8049: f64, t8052: f64, t8055: f64, t8058: f64, t8083: f64, t8087: f64, t8091: f64, t8095: f64) -> f64 {
    let t8104 = -0.34752604166666666667e-3_f64 * t8083 * t2197 + 0.46377350260416666667e-4_f64 * t7772 * t8087 - t7786 - 0.11584201388888888889e-3_f64 * t7788 * t8091 + 0.34752604166666666667e-3_f64 * t7788 * t8095 + 0.34752604166666666667e-3_f64 * t7788 * t8087 + t7801 + 0.11607361111111111111e-2_f64 * t8049 + 0.17411041666666666666e-2_f64 * t8052 - 0.17411041666666666666e-2_f64 * t8055 + 0.11607361111111111111e-2_f64 * t8058;
    t8104
}
