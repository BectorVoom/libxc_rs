//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1158/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1158(t11545: f64, t40276: f64, t983: f64, t986: f64, t792: f64, t40594: f64, t4176: f64, t39299: f64, t3275: f64, t3276: f64, t12428: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t42845 = 5.0_f64 / 8.0_f64 * t40276 * t11545;
    let t42846 = t983 * t986;
    let t42847 = t42846 * t792;
    let t42850 = 45.0_f64 / 32.0_f64 * t40594 * t4176 * t42847;
    let t42851 = t39299 * t986;
    let t42854 = 5.0_f64 / 8.0_f64 * t3275 * t3276 * t42851;
    let t42855 = t12428 * t481;
    (t42845, t42846, t42850, t42854, t42855)
}
