//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2490/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2490(t1247: f64, t1251: f64, t42994: f64, t1032: f64, t1246: f64, t12690: f64, t12904: f64, t3708: f64, t11262: f64, t3590: f64, t3610: f64, t3612: f64) -> (f64, f64, f64, f64, f64) {
    let t44264 = t1247 * t42994 * t1251;
    let t44267 = t12690 * t1032 * t1246;
    let t44270 = t3708 * t12904;
    let t44273 = t1247 * t11262 * t3590;
    let t44276 = t3610 * t11262 * t3612;
    (t44264, t44267, t44270, t44273, t44276)
}
