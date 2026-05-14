//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 423/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk423<F: Float>(t2607: F, t5170: F, t2606: F, t265: F, t4969: F, t724: F, t1901: F, t193: F, t2553: F, t3835: F, t3958: F, t3986: F, t3988: F, t446: F, t5066: F, t5070: F, t5075: F, t5079: F, t5083: F, t5087: F, t5134: F, t5149: F, t5153: F, t5157: F, t5161: F, t5167: F, t89: F) -> (F, F, F, F) {
    let t5171 = t2607 * t5170;
    let t5172 = t2606 * t5171;
    let t5176 = t724 * t265 * t4969;
    let t5179 = -2.0 / 9.0 * t3958 + 2.0 / 3.0 * t446 * t5066 + 2.0 / 3.0 * t446 * t5070 + 2.0 / 3.0 * t446 * t5075 - 2.0 / 9.0 * t446 * t5079 - t446 * t5083 / 9.0 - 2.0 / 27.0 * t446 * t5087 + 2.0 / 9.0 * t3986 + t2553 + 2.0 / 9.0 * t3988 + t89 * t193 * t5134 / 3.0 - t446 * t5149 / 3.0 - 2.0 / 3.0 * t446 * t5153 - 2.0 / 3.0 * t446 * t5157 - t446 * t5161 / 3.0 + 2.0 / 27.0 * t3835 + 2.0 / 9.0 * t1901 * t5167 + 2.0 / 9.0 * t1901 * t5172 + 2.0 / 9.0 * t446 * t5176;
    (t5171, t5172, t5176, t5179)
}
