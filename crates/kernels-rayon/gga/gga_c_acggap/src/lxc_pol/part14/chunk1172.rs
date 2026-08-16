//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1172/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1172(t1988: f64, t9691: f64, t1742: f64, t1980: f64, t1982: f64, t1992: f64, t5: f64, t31570: f64, t31593: f64, t31598: f64, t31602: f64, t35775: f64, t35785: f64, t35789: f64, t35795: f64, t35798: f64, t35800: f64, t37719: f64, t40145: f64, t40147: f64, t40152: f64, t40156: f64) -> f64 {
    let t40158 = t1988 * t9691;
    let t40163 = t1980 * t1982 * t5 * t1742 * t1992;
    let t40165 = 0.31448092289604152068e-3_f64 * t31570 - 0.21437009059034868486e-3_f64 * t31593 - t31598 - t31602 + t35775 + t35785 + t35789 + t37719 - t35795 + t35798 + t35800 + 0.17149607247227894789e-2_f64 * t40145 + 7.0_f64 / 144.0_f64 * t40147 + 0.10718504529517434243e-3_f64 * t40152 + 0.7145669686344956162e-4_f64 * t40156 - 0.31448092289604152068e-3_f64 * t40158 - 0.20965394859736101379e-3_f64 * t40163;
    t40165
}
