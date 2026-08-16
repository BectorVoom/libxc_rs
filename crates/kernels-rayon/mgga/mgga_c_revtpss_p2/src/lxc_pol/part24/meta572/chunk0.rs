//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1751/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1751(t1196: f64, t12552: f64, t3523: f64, t90357: f64, t12248: f64, t6470: f64, t6474: f64, t1732: f64, t24324: f64, t3384: f64, t3433: f64, t81650: f64) -> (f64, f64, f64, f64) {
    let t90361 = 0.6233709278045326953e3_f64 * t1196 * t12552 * t90357 * t3523;
    let t90364 = 0.57895126195293126241e3_f64 * t12248 * t6474 * t6470;
    let t90367 = 8.0_f64 * t3384 * t24324 * t1732;
    let t90370 = 0.64327917994770140268e2_f64 * t3433 * t81650 * t1732;
    (t90361, t90364, t90367, t90370)
}
