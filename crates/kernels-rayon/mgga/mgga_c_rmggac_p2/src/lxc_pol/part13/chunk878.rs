//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 878/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk878(t35018: f64, t8571: f64, t36740: f64, t9222: f64, t118: f64, t128: f64, t1494: f64, t1986: f64, t209: f64, t7474: f64, t1970: f64, t1971: f64, t236: f64, t5615: f64) -> (f64, f64, f64, f64) {
    let t39497 = t8571 * t35018;
    let t39499 = t9222 * t36740;
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39514 = t7474 * t39513;
    let t39518 = t1970 * t1971 * t236 * t5615;
    (t39497, t39499, t39514, t39518)
}
