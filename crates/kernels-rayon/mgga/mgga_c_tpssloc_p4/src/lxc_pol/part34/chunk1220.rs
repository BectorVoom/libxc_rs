//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1220/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1220(t101226: f64, t105755: f64, t105759: f64, t105763: f64, t105766: f64, t105780: f64, t105797: f64, t105814: f64, t105818: f64, t105822: f64, t1877: f64, t2057: f64, t24191: f64, t24344: f64, t2522: f64, t26563: f64, t26744: f64, t28252: f64, t28256: f64, t28456: f64, t28459: f64, t28462: f64, t7114: f64, t7545: f64, t7845: f64, t84766: f64, t93000: f64) -> f64 {
    let t108096 = 9.0_f64 * t2522 * t7845 * t28252 - 3.0_f64 / 2.0_f64 * t1877 * t101226 * t7545 - 9.0_f64 * t24191 * t105766 - 9.0_f64 / 2.0_f64 * t24191 * t105759 + 9.0_f64 / 2.0_f64 * t2522 * t7845 * t28256 - 3.0_f64 * t1877 * t84766 * t105822 - 3.0_f64 * t1877 * t26744 * t28459 - 3.0_f64 / 2.0_f64 * t1877 * t26744 * t28462 - 9.0_f64 / 2.0_f64 * t24191 * t105755 + 3.0_f64 * t1877 * t24344 * t105814 - 3.0_f64 / 2.0_f64 * t1877 * t7114 * t105818 + 3.0_f64 * t1877 * t93000 * t28456 - 9.0_f64 * t26563 * t105763 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t105797 - 3.0_f64 / 2.0_f64 * t1877 * t7114 * t105780;
    t108096
}
