//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 969/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk969(t10375: f64, t38: f64, t10314: f64, t10317: f64, t10320: f64, t1291: f64, t1307: f64, t1314: f64, t1986: f64, t1994: f64, t1997: f64, t2046: f64, t3441: f64, t3463: f64, t3483: f64, t583: f64, t616: f64, t85: f64) -> f64 {
    let t10376 = t38 * t10375;
    let t10383 = -t1291 * t2046 / 12.0_f64 - t10314 * t85 / 12.0_f64 - t10317 * t85 / 12.0_f64 - t10320 * t85 / 6.0_f64 - t3441 * t616 / 6.0_f64 - t1986 * t1314 / 12.0_f64 - t1994 * t1314 / 12.0_f64 - t1997 * t1314 / 6.0_f64 - t583 * t3483 / 6.0_f64 + t10376 * t85 / 24.0_f64 + t3463 * t616 / 12.0_f64 + t1307 * t2046 / 24.0_f64;
    t10383
}
