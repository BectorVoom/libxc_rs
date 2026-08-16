//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1084/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1084<F: Float>(t1934: F, t2347: F, t2469: F, t2492: F, t10009: F, t8392: F, t2399: F, t2544: F, t89: F, t10007: F, t10008: F, t10044: F, t10085: F, t14163: F, t14175: F, t14182: F, t14187: F, t1901: F, t2360: F, t242: F, t2608: F, t41419: F, t41441: F, t446: F, t684: F, t724: F, t773: F, t9587: F, t9798: F, t9838: F) -> (F, F) {
    let t42570 = t2347 * t1934;
    let t42575 = t2492 * t2469;
    let t42583 = t8392 * t10009;
    let t42599 = t89 * t2399 * t2544;
    let t42605 = -F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t14182 * t2360 * t1934 * t2608 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t14187 * t42570 * t2608 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t42575 * t10008 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t14175 * t9838 * t684 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t42583 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t14163 * t41441 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t10007 * t10044 * t684 - F::cast_from(2.0_f64) * t446 * t242 * t41419 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t10085 * t9798 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t42599 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t446 * t724 * t773 * t9587;
    (t42570, t42605)
}
