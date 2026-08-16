//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1152/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1152<F: Float>(t1882: F, t36123: F, t36242: F, t34172: F, t55768: F, t10683: F, t144005: F, t15195: F, t15254: F, t152776: F, t152799: F, t15369: F, t1901: F, t2360: F, t24898: F, t2749: F, t2862: F, t29399: F, t296: F, t319: F, t34065: F, t34208: F, t36240: F, t3886: F, t4146: F, t4246: F, t4299: F, t446: F, t56418: F, t7584: F, t7679: F, t840: F, t871: F) -> (F, F, F) {
    let t153932 = t1882 * t36123;
    let t153970 = t1882 * t36242;
    let t153976 = t55768 * t34172;
    let t154001 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t15369 * t24898 * t29399 + t1901 * t15195 * t34208 / F::cast_from(9.0_f64) + t1901 * t144005 * t4146 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t153970 + t446 * t840 * t4246 * t34065 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) * t446 * t296 * t153976 - F::cast_from(2.0_f64) * t446 * t10683 * t319 * t152776 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2862 * t2749 * t36240 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2862 * t871 * t7584 * t4299 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t56418 * t152799 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t15254 * t7679 * t2360 * t3886;
    (t153932, t153976, t154001)
}
