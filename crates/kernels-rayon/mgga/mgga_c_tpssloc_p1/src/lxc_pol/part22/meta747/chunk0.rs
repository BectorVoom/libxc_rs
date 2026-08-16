//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2488/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2488(t13969: f64, t21486: f64, t3130: f64, t1041: f64, t13995: f64, t17705: f64, t17976: f64, t18036: f64, t21512: f64, t3117: f64, t43219: f64, t4582: f64, t4588: f64, t4644: f64, t49929: f64, t50175: f64, t50181: f64, t62631: f64, t62640: f64, t70316: f64) -> f64 {
    let t70805 = t3130 * t13969 * t21486;
    let t70823 = t70805 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t3117 * t21512 + 5.0_f64 / 4608.0_f64 * t1041 * t4582 * t4588 * t70316 - t4644 * t17976 / 384.0_f64 - t50175 + t50181 / 3456.0_f64 + t13995 * t17705 / 768.0_f64 + t49929 * t18036 / 768.0_f64 - t62631 / 72.0_f64 + t62640 / 48.0_f64 + t43219 / 10368.0_f64;
    t70823
}
