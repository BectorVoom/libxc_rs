//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2116/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2116(t10143: f64, t5664: f64, t12895: f64, t13121: f64, t1484: f64, t16697: f64, t16699: f64, t16700: f64, t16703: f64, t16705: f64, t16707: f64, t16708: f64, t16709: f64, t16712: f64, t16715: f64, t16719: f64, t1877: f64, t193: f64, t2522: f64, t262: f64, t5527: f64, t776: f64, t868: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64) -> (f64, f64) {
    let t17120 = t5664 * t10143;
    let t17131 = 6.0_f64 * t193 * t262 * t5527 * t776 + 6.0_f64 * t12895 * t1484 * t2522 + 2.0_f64 * t17120 * t1877 * t868 - t13121 - t16697 + t16699 - t16700 + t16703 + t16705 + t16707 - t16708 + t16709 - t16712 + t16715 + t16719 + t9853 + t9859 - t9894 + t9907 - t9921;
    (t17120, t17131)
}
