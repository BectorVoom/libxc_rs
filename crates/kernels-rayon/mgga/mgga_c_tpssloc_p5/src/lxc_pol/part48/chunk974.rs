//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 974/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk974(t113741: f64, t114971: f64, t114988: f64, t114992: f64, t115027: f64, t115030: f64, t1877: f64, t1914: f64, t23792: f64, t23796: f64, t23807: f64, t23813: f64, t24191: f64, t24339: f64, t2522: f64, t25927: f64, t26756: f64, t28: f64, t30974: f64, t31430: f64, t31434: f64, t31496: f64, t31502: f64, t3231: f64, t6841: f64, t6848: f64, t7114: f64, t84797: f64, t8566: f64, t92271: f64) -> f64 {
    let t115184 = 2.0_f64 * t92271 * t31502 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t23796 - t1877 * t7114 * t3231 * t1914 / 2.0_f64 + 6.0_f64 * t24191 * t25927 * t115030 + 3.0_f64 * t2522 * t31430 * t6841 - t1877 * t7114 * t113741 / 2.0_f64 + t1877 * t115027 * t23807 + t26756 * t25927 * t114988 + t1877 * t114971 * t28 / 2.0_f64 - t1877 * t114992 * t6848 + t1877 * t8566 * t3231 / 2.0_f64 - t1877 * t24339 * t30974 - t1877 * t31434 * t23813 / 2.0_f64 + 3.0_f64 * t2522 * t8566 * t23792 - 3.0_f64 * t84797 * t31496;
    t115184
}
