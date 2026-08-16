//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 796/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk796(t3154: f64, t38351: f64, t1494: f64, t1970: f64, t1971: f64, t209: f64, t515: f64, t664: f64, t3352: f64, t70423: f64, t8456: f64, t14225: f64, t7248: f64, t9170: f64) -> (f64, f64, f64, f64) {
    let t74356 = t38351 * t3154;
    let t74368 = t1970 * t1971 * t515 * t664 * t1494 * t209;
    let t74371 = t70423 * t3352 * t8456;
    let t74374 = t14225 * t7248 * t9170;
    (t74356, t74368, t74371, t74374)
}
