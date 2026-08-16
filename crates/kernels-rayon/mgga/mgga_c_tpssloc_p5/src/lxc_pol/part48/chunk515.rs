//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 515/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk515(t1332: f64, t1336: f64, t1381: f64, t1383: f64, t3773: f64, t3777: f64, t3898: f64, t3902: f64, t3905: f64, t3907: f64, t3909: f64, t544: f64, t564: f64) -> f64 {
    let t3911 = 2.0_f64 * t1332 * t1383 + 2.0_f64 * t1336 * t3898 - 2.0_f64 * t1336 * t3902 - t1336 * t3905 - t1336 * t3907 - 2.0_f64 * t1381 * t3777 + t3773 * t564 + t3909 * t544;
    t3911
}
