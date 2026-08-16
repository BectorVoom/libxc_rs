//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 925/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk925(t8392: f64, t8559: f64, t488: f64, t8216: f64, t1882: f64, t8238: f64, t8365: f64, t8512: f64, t8507: f64, t8373: f64, t480: f64, t8369: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39118 = t8392 * t8559;
    let t39120 = t8216 * t488;
    let t39135 = t1882 * t8238;
    let t39137 = t1882 * t8365;
    let t39143 = t8392 * t8512;
    let t39145 = t8392 * t8507;
    let t39147 = t8392 * t8373;
    let t39150 = t8216 * t480;
    let t39154 = t8392 * t8369;
    (t39118, t39120, t39135, t39137, t39143, t39145, t39147, t39150, t39154)
}
