//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1161/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1161(t2347: f64, t2842: f64, t2878: f64, t38953: f64, t2885: f64, t10458: f64, t8392: f64, t865: f64, t89: f64, t9555: f64, t10500: f64, t10388: f64, t10443: f64, t10447: f64, t10457: f64, t10479: f64, t10504: f64, t15402: f64, t1901: f64, t193: f64, t2874: f64, t295: f64, t312: f64, t4139: f64, t43944: f64, t43999: f64, t44205: f64, t684: f64) -> f64 {
    let t44566 = t2842 * t2347;
    let t44571 = t38953 * t2878;
    let t44573 = t38953 * t2885;
    let t44575 = t8392 * t10458;
    let t44583 = t89 * t9555 * t865;
    let t44585 = t8392 * t10500;
    let t44587 = -16.0_f64 / 9.0_f64 * t1901 * t10479 * t15402 * t43999 - 8.0_f64 / 3.0_f64 * t1901 * t10447 * t10504 + 4.0_f64 / 3.0_f64 * t1901 * t10443 * t10457 + 4.0_f64 / 9.0_f64 * t1901 * t2874 * t312 * t10388 * t684 - 8.0_f64 / 9.0_f64 * t1901 * t4139 * t44566 * t44205 + 16.0_f64 / 27.0_f64 * t44571 + 16.0_f64 / 27.0_f64 * t44573 - 4.0_f64 / 9.0_f64 * t44575 + t89 * t193 * t295 * t43944 * t312 / 3.0_f64 - 112.0_f64 / 81.0_f64 * t44583 - 4.0_f64 / 9.0_f64 * t44585;
    t44587
}
