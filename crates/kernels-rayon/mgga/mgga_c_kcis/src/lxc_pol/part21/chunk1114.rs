//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1114/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1114(t27055: f64, t7772: f64, t26779: f64, t26787: f64, t26798: f64, t26801: f64, t26804: f64, t26812: f64, t26826: f64, t26829: f64, t26834: f64, t26838: f64, t27020: f64, t27042: f64, t27053: f64, t7775: f64) -> (f64, f64) {
    let t27056 = t7772 * t27055;
    let t27058 = 0.92858888888888888886e-2_f64 * t26779 + 0.15476481481481481481e-2_f64 * t26787 - 0.34822083333333333332e-2_f64 * t26798 - 0.24734586805555555556e-3_f64 * t27042 * t7775 - 0.23214722222222222222e-2_f64 * t26801 + 0.15476481481481481481e-2_f64 * t26804 + 0.46377350260416666667e-4_f64 * t7772 * t27020 - 0.17411041666666666666e-2_f64 * t26812 + 0.23214722222222222222e-2_f64 * t26826 + 0.17411041666666666666e-2_f64 * t26829 + 0.17024129629629629629e-1_f64 * t26834 - t27053 - 0.61905925925925925925e-2_f64 * t26838 + 0.30918233506944444444e-4_f64 * t27056;
    (t27056, t27058)
}
