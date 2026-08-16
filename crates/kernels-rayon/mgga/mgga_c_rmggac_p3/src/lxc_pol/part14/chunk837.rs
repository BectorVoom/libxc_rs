//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 837/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk837(t16503: f64, t35039: f64, t352: f64, t38422: f64, t38649: f64, t1652: f64, t7778: f64, t739: f64, t1550: f64, t2060: f64, t27124: f64, t8542: f64, t9128: f64) -> (f64, f64, f64, f64, f64) {
    let t38663 = t16503 * t35039 * t38422 * t38649 * t352;
    let t38674 = t7778 * t1652;
    let t38675 = t739 * t38674;
    let t38676 = 0.79828278012425390426e-1_f64 * t38675;
    let t38678 = t1550 * t2060 * t27124;
    let t38680 = t9128 * t8542;
    (t38663, t38674, t38676, t38678, t38680)
}
