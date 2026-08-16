//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2601/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2601(t13969: f64, t15530: f64, t3515: f64, t11702: f64, t5002: f64, t11708: f64, t15502: f64, t15506: f64, t15554: f64, t3506: f64, t10469: f64, t1720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52795 = t3515 * t13969 * t15530;
    let t52801 = t5002 * t11702;
    let t52810 = t11708 * t15502;
    let t52813 = t11708 * t15506;
    let t52817 = t3506 * t13969 * t15554;
    let t52834 = t1720 * t10469;
    (t52795, t52801, t52810, t52813, t52817, t52834)
}
