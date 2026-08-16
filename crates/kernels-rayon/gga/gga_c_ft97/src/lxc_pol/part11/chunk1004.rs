//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1004/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1004(t157: f64, t40436: f64, t604: f64, t7763: f64, t2101: f64, t2142: f64, t12709: f64, t12714: f64, t12723: f64, t12724: f64, t12982: f64, t144: f64, t1901: f64, t1986: f64, t2157: f64, t2185: f64, t2210: f64, t2211: f64, t2212: f64, t38688: f64, t38693: f64, t38930: f64, t40519: f64, t40700: f64, t40772: f64, t446: f64, t605: f64, t609: f64, t7745: f64, t9017: f64, t9145: f64, t9362: f64, t9432: f64) -> f64 {
    let t40926 = t40436 * t157;
    let t40931 = t604 * t7763;
    let t40945 = t2101 * t2142;
    let t40970 = -8.0_f64 / 9.0_f64 * t1901 * t12982 * t9362 + 40.0_f64 / 81.0_f64 * t1901 * t40926 * t12724 * t40700 + 40.0_f64 / 81.0_f64 * t1901 * t12723 * t40931 * t40772 - 20.0_f64 / 27.0_f64 * t1901 * t12723 * t12724 * t38930 + 4.0_f64 / 9.0_f64 * t1901 * t2210 * t2211 * t7745 * t609 - 8.0_f64 / 3.0_f64 * t1901 * t40945 * t9145 - 8.0_f64 / 3.0_f64 * t1901 * t12709 * t38688 * t2212 + 8.0_f64 / 9.0_f64 * t1901 * t12714 * t38693 * t2212 - 4.0_f64 / 3.0_f64 * t446 * t144 * t40519 - 4.0_f64 * t446 * t2185 * t605 * t1986 * t2157 + 8.0_f64 * t446 * t9432 * t605 * t9017 * t609;
    t40970
}
