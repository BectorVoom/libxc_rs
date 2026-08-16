//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 970/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk970(t11336: f64, t3270: f64, t795: f64, t3269: f64, t1115: f64, t481: f64, t10667: f64, t3493: f64, t10630: f64, t3262: f64, t3465: f64, t11020: f64, t3469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11338 = t3270 * t11336 * t795;
    let t11339 = t3269 * t11338;
    let t11340 = t11339 / 2.0_f64;
    let t11342 = t3270 * t1115 * t481;
    let t11343 = t10667 * t11342;
    let t11344 = 3.0_f64 / 2.0_f64 * t11343;
    let t11345 = t3270 * t3493;
    let t11346 = t3269 * t11345;
    let t11347 = t11346 / 2.0_f64;
    let t11349 = t3262 * t3465 * t10630;
    let t11350 = 3.0_f64 / 4.0_f64 * t11349;
    let t11351 = t11020 * t3469;
    (t11338, t11340, t11342, t11344, t11345, t11347, t11350, t11351)
}
