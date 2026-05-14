//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 898/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk898<F: Float>(t2198: F, t8232: F, t1882: F, t9333: F, t9324: F, t9337: F, t9329: F, t9295: F, t9434: F, t143: F, t38052: F, t9408: F, t1637: F, t2135: F, t89: F, t13208: F, t1651: F, t167: F, t1901: F, t2075: F, t2142: F, t2157: F, t2185: F, t2230: F, t38053: F, t38057: F, t39664: F, t446: F, t569: F, t574: F, t605: F, t609: F, t616: F, t7973: F, t9007: F, t9304: F, t9316: F) -> (F, F, F, F, F, F, F, F) {
    let t40979 = t8232 * t2198;
    let t40981 = t1882 * t9333;
    let t40983 = t1882 * t9324;
    let t40985 = t1882 * t9337;
    let t40987 = t1882 * t9329;
    let t40989 = t1882 * t9295;
    let t40991 = t1882 * t9434;
    let t41002 = t38052 * t143;
    let t41019 = t1882 * t9408;
    let t41040 = t89 * t1637 * t2135;
    let t41045 = -80.0 / 243.0 * t446 * t41002 * t167 * t38053 - 2.0 / 3.0 * t446 * t569 * t2230 * t1651 - 4.0 / 9.0 * t446 * t569 * t616 * t7973 - t446 * t569 * t167 * t38057 / 9.0 + 4.0 / 9.0 * t41019 + 2.0 * t446 * t574 * t605 * t2075 * t2157 + 4.0 * t446 * t574 * t2142 * t9316 + 4.0 / 3.0 * t446 * t574 * t605 * t9007 * t609 - 8.0 * t446 * t2185 * t2142 * t9304 + 8.0 / 9.0 * t41040 - 8.0 / 3.0 * t1901 * t13208 * t39664;
    (t40979, t40981, t40983, t40985, t40987, t40989, t40991, t41045)
}
