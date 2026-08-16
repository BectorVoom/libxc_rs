//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 847/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk847(t1971: f64, t27177: f64, t3351: f64, t7190: f64, t615: f64, t7230: f64, t875: f64, t876: f64, t16156: f64, t8812: f64, t2320: f64, t35265: f64) -> (f64, f64, f64, f64) {
    let t38991 = t3351 * t1971 * t7190 * t27177;
    let t38996 = t7230 * t1971 * t875 * t615 * t876;
    let t38998 = t16156 * t8812;
    let t39003 = t35265 * t2320;
    (t38991, t38996, t38998, t39003)
}
