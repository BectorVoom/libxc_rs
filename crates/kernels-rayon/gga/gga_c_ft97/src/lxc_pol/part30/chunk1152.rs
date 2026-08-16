//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1152/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1152(t1882: f64, t36123: f64, t36242: f64, t34172: f64, t55768: f64, t10683: f64, t144005: f64, t15195: f64, t15254: f64, t152776: f64, t152799: f64, t15369: f64, t1901: f64, t2360: f64, t24898: f64, t2749: f64, t2862: f64, t29399: f64, t296: f64, t319: f64, t34065: f64, t34208: f64, t36240: f64, t3886: f64, t4146: f64, t4246: f64, t4299: f64, t446: f64, t56418: f64, t7584: f64, t7679: f64, t840: f64, t871: f64) -> (f64, f64, f64) {
    let t153932 = t1882 * t36123;
    let t153970 = t1882 * t36242;
    let t153976 = t55768 * t34172;
    let t154001 = -4.0_f64 / 3.0_f64 * t1901 * t15369 * t24898 * t29399 + t1901 * t15195 * t34208 / 9.0_f64 + t1901 * t144005 * t4146 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t153970 + t446 * t840 * t4246 * t34065 / 3.0_f64 - 2.0_f64 * t446 * t296 * t153976 - 2.0_f64 * t446 * t10683 * t319 * t152776 - 2.0_f64 / 3.0_f64 * t446 * t2862 * t2749 * t36240 - 2.0_f64 / 3.0_f64 * t446 * t2862 * t871 * t7584 * t4299 + 2.0_f64 / 3.0_f64 * t1901 * t56418 * t152799 - 2.0_f64 / 9.0_f64 * t1901 * t15254 * t7679 * t2360 * t3886;
    (t153932, t153976, t154001)
}
