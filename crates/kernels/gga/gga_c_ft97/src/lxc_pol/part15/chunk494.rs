//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 494/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk494<F: Float>(t1901: F, t193: F, t2553: F, t3835: F, t3958: F, t3986: F, t3988: F, t446: F, t5066: F, t5070: F, t5075: F, t5079: F, t5083: F, t5087: F, t5134: F, t5149: F, t5153: F, t5157: F, t5161: F, t5167: F, t5172: F, t5176: F, t89: F) -> (F,) {
    let t5179 = -2.0 / 9.0 * t3958 + 2.0 / 3.0 * t446 * t5066 + 2.0 / 3.0 * t446 * t5070 + 2.0 / 3.0 * t446 * t5075 - 2.0 / 9.0 * t446 * t5079 - t446 * t5083 / 9.0 - 2.0 / 27.0 * t446 * t5087 + 2.0 / 9.0 * t3986 + t2553 + 2.0 / 9.0 * t3988 + t89 * t193 * t5134 / 3.0 - t446 * t5149 / 3.0 - 2.0 / 3.0 * t446 * t5153 - 2.0 / 3.0 * t446 * t5157 - t446 * t5161 / 3.0 + 2.0 / 27.0 * t3835 + 2.0 / 9.0 * t1901 * t5167 + 2.0 / 9.0 * t1901 * t5172 + 2.0 / 9.0 * t446 * t5176;
    (t5179,)
}
