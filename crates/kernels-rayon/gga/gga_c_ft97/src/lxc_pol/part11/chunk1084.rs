//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1084/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1084(t1934: f64, t2347: f64, t2469: f64, t2492: f64, t10009: f64, t8392: f64, t2399: f64, t2544: f64, t89: f64, t10007: f64, t10008: f64, t10044: f64, t10085: f64, t14163: f64, t14175: f64, t14182: f64, t14187: f64, t1901: f64, t2360: f64, t242: f64, t2608: f64, t41419: f64, t41441: f64, t446: f64, t684: f64, t724: f64, t773: f64, t9587: f64, t9798: f64, t9838: f64) -> (f64, f64) {
    let t42570 = t2347 * t1934;
    let t42575 = t2492 * t2469;
    let t42583 = t8392 * t10009;
    let t42599 = t89 * t2399 * t2544;
    let t42605 = -8.0_f64 / 3.0_f64 * t1901 * t14182 * t2360 * t1934 * t2608 + 8.0_f64 / 9.0_f64 * t1901 * t14187 * t42570 * t2608 - 8.0_f64 / 3.0_f64 * t1901 * t42575 * t10008 - 8.0_f64 / 3.0_f64 * t1901 * t14175 * t9838 * t684 + 8.0_f64 / 9.0_f64 * t42583 - 8.0_f64 / 3.0_f64 * t1901 * t14163 * t41441 - 4.0_f64 / 3.0_f64 * t1901 * t10007 * t10044 * t684 - 2.0_f64 * t446 * t242 * t41419 - 8.0_f64 / 3.0_f64 * t1901 * t10085 * t9798 + 8.0_f64 / 9.0_f64 * t42599 - 8.0_f64 / 3.0_f64 * t446 * t724 * t773 * t9587;
    (t42570, t42605)
}
