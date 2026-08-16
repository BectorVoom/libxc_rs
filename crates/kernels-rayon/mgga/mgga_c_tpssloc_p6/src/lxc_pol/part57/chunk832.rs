//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 832/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk832(t6420: f64, t7208: f64, t6415: f64, t1825: f64, t27097: f64, t1336: f64, t24108: f64, t24110: f64, t26427: f64, t26429: f64, t26437: f64, t28161: f64, t28165: f64, t28169: f64, t28183: f64, t5234: f64, t7932: f64) -> f64 {
    let t29343 = t7208 * t6420;
    let t29345 = t7208 * t6415;
    let t29349 = t27097 * t1825;
    let t29359 = -t1336 * t29343 - t1336 * t29345 - 2.0_f64 * t5234 * t7932 - 2.0_f64 * t1336 * t29349 + 0.16449340668482264365e-1_f64 * t26427 - 0.76763589786250567036e-1_f64 * t26429 - 0.16449340668482264365e-1_f64 * t26437 + 0.16449340668482264365e-1_f64 * t28161 + t24108 + t24110 - 0.3289868133696452873e-1_f64 * t28165 - 0.16449340668482264365e-1_f64 * t28169 - 0.16449340668482264365e-1_f64 * t28183;
    t29359
}
