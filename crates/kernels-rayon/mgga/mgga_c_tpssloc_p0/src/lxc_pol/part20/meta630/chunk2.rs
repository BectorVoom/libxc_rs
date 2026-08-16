//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2286/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2286(t12971: f64, t13141: f64, t13151: f64, t13160: f64, t13161: f64, t13164: f64, t13167: f64, t1504: f64, t16729: f64, t1891: f64, t232: f64, t2379: f64, t2553: f64, t2667: f64, t4119: f64, t4225: f64, t4227: f64, t47213: f64, t68: f64, t776: f64, t822: f64, t825: f64, t845: f64, t9947: f64, t9951: f64) -> f64 {
    let t47215 = (-36.0_f64 * t12971 * t4225 * t776 * t845 + 180.0_f64 * t1891 * t2379 * t4119 * t4225 - 36.0_f64 * t13160 * t2553 * t4225 - 36.0_f64 * t2667 * t4227 * t68 + 9.0_f64 * t13141 * t825 - 72.0_f64 * t13151 * t13161 - 36.0_f64 * t13151 * t13164 + 9.0_f64 * t13167 * t822 + 60.0_f64 * t1504 * t9947 - 36.0_f64 * t16729 * t9951 + t47213) * t232;
    t47215
}
