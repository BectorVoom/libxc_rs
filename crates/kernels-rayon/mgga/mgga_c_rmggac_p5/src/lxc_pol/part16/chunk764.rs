//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 764/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk764(t1343: f64, t2040: f64, t638: f64, t71: f64, t830: f64, t2046: f64, t2051: f64, t271: f64, t4773: f64, t7292: f64, t7385: f64, t2067: f64, t25640: f64) -> (f64, f64, f64, f64) {
    let t35781 = t638 * t830 * t1343 * t71 * t2040;
    let t35786 = t2046 * t4773 * t271 * t71 * t2051;
    let t35798 = t638 * t7292 * t7385;
    let t35810 = t25640 * t2067;
    (t35781, t35786, t35798, t35810)
}
