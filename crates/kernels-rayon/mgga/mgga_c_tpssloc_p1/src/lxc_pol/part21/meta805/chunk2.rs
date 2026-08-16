//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2795/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2795(t12971: f64, t13141: f64, t13151: f64, t13157: f64, t13161: f64, t13167: f64, t1504: f64, t1506: f64, t16662: f64, t16729: f64, t16736: f64, t16740: f64, t16745: f64, t16746: f64, t225: f64, t230: f64, t2379: f64, t2553: f64, t2672: f64, t4225: f64, t4226: f64, t5527: f64, t5601: f64, t58963: f64, t58964: f64, t58966: f64, t58967: f64, t58970: f64, t58981: f64, t59010: f64, t59050: f64, t6589: f64, t776: f64, t845: f64) -> f64 {
    let t59072 = 6.0_f64 * t13141 * t1506 - 48.0_f64 * t16729 * t13161 + 6.0_f64 * t1504 * t13167 - 24.0_f64 * t4225 * t845 * t16662 * t776 - 12.0_f64 * t4225 * t16745 * t2553 - 24.0_f64 * t13151 * t16746 - (t58963 + t58964 + t58966 + t58967 + t58970 + t58981 + t59010 + t59050) * t225 * t230 + 60.0_f64 * t4225 * t16736 * t2553 - 24.0_f64 * t4225 * t4226 * t12971 - 12.0_f64 * t5601 * t2672 - 48.0_f64 * t13151 * t16740 + 120.0_f64 * t16729 * t13157 - 360.0_f64 * t4225 * t6589 * t5527 * t2379;
    t59072
}
