//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 834/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk834<F: Float>(t8392: F, t8559: F, t488: F, t8216: F, t1882: F, t8238: F, t8365: F, t8512: F, t8507: F, t8373: F, t480: F, t8369: F, t8396: F, t8326: F, t8583: F, t8574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39118 = t8392 * t8559;
    let t39120 = t8216 * t488;
    let t39135 = t1882 * t8238;
    let t39137 = t1882 * t8365;
    let t39143 = t8392 * t8512;
    let t39145 = t8392 * t8507;
    let t39147 = t8392 * t8373;
    let t39150 = t8216 * t480;
    let t39154 = t8392 * t8369;
    let t39161 = t1882 * t8396;
    let t39167 = t8326 * t488;
    let t39183 = t1882 * t8583;
    let t39185 = t1882 * t8574;
    (t39118, t39120, t39135, t39137, t39143, t39145, t39147, t39150, t39154, t39161, t39167, t39183, t39185)
}
