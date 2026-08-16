//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 877/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk877(t34847: f64, t9118: f64, t16156: f64, t9111: f64, t3351: f64, t618: f64, t7231: f64, t875: f64, t876: f64, t839: f64, t880: f64, t236: f64, t35155: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t39231 = t34847 * t9118;
    let t39233 = t16156 * t9111;
    let t39234 = 0.19863479950205658386e-4_f64 * t39233;
    let t39238 = t3351 * t7231 * t875 * t618 * t876;
    let t39243 = t3351 * t7231 * t880 * t618 * t839;
    let t39248 = t3351 * t35155 * t236 * t618 * t794;
    (t39231, t39234, t39238, t39243, t39248)
}
