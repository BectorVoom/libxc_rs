//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1154/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1154(t322: f64, t42547: f64, t12692: f64, t1338: f64, t1348: f64, t10533: f64, t11145: f64, t11157: f64, t12002: f64, t12009: f64, t12683: f64, t2438: f64, t3413: f64, t3420: f64, t352: f64, t35220: f64, t3675: f64, t37204: f64, t37223: f64, t41028: f64, t41042: f64, t855: f64, t9760: f64) -> f64 {
    let t332 = 0.25e1_f64 < t322;
    let t42775 = piecewise3(t332, t42547, 0.0_f64);
    let t42791 = t1338 * t12692;
    let t42794 = t1348 * t12692;
    let t42807 = -0.105e1_f64 * t855 * t42775 * t352 - 0.63e1_f64 * t11157 * t12683 - 0.42e1_f64 * t41028 * t3675 - 0.42e1_f64 * t12002 * t9760 - 0.945e1_f64 * t37223 * t12683 - 0.21e1_f64 * t11145 * t10533 - 0.21e1_f64 * t3413 * t35220 - 0.21e1_f64 * t42791 * t2438 - 0.1575e1_f64 * t42794 * t2438 - 0.315e1_f64 * t41042 * t3675 - 0.315e1_f64 * t12009 * t9760 - 0.1575e1_f64 * t11157 * t10533 - 0.1575e1_f64 * t3420 * t35220 - 0.23625e1_f64 * t37204 * t12683;
    t42807
}
