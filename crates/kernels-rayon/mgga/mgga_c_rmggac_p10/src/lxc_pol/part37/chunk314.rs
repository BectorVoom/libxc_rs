//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 314/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk314(t21: f64, t40: f64, t1318: f64, t2045: f64) -> (f64, f64, f64, f64, f64) {
    let t3051 = t21 * t21;
    let t3052 = f64::sqrt(t40);
    let t3054 = 1.0_f64 / t3052 / t1318;
    let t3055 = t3051 * t3054;
    let t3056 = t3055 * t2045;
    (t3051, t3052, t3054, t3055, t3056)
}
