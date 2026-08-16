//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1161/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1161<F: Float>(t2347: F, t2842: F, t2878: F, t38953: F, t2885: F, t10458: F, t8392: F, t865: F, t89: F, t9555: F, t10500: F, t10388: F, t10443: F, t10447: F, t10457: F, t10479: F, t10504: F, t15402: F, t1901: F, t193: F, t2874: F, t295: F, t312: F, t4139: F, t43944: F, t43999: F, t44205: F, t684: F) -> F {
    let t44566 = t2842 * t2347;
    let t44571 = t38953 * t2878;
    let t44573 = t38953 * t2885;
    let t44575 = t8392 * t10458;
    let t44583 = t89 * t9555 * t865;
    let t44585 = t8392 * t10500;
    let t44587 = -F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t1901 * t10479 * t15402 * t43999 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t10447 * t10504 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t10443 * t10457 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t2874 * t312 * t10388 * t684 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t4139 * t44566 * t44205 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t44571 + F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t44573 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t44575 + t89 * t193 * t295 * t43944 * t312 / F::cast_from(3.0_f64) - F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t44583 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t44585;
    t44587
}
