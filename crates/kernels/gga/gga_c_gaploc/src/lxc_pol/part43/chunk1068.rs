//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1068/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1068<F: Float>(t41544: F, t44156: F, t44157: F, t44162: F, t44164: F, t44170: F, t44174: F, t44178: F, t44179: F, t44180: F, t44181: F, t44185: F, t44186: F, t47562: F, t47564: F, t47567: F, t47572: F, t47574: F, t47575: F, t47576: F) -> F {
    let t51188 = -t44156 - t44157 - t47562 - F::new(0.18404604457881959845e2) * t47564 + F::new(0.87421871174939309263e2) * t47567 - t44162 - t44164 - t44170 - t44174 - t44178 + F::new(0.71500979903700853338e0) * t47572 - t44179 - t44180 + t44181 - t47574 + t47575 - t47576 + t44185 + t44186 - F::new(0.76685851907841499353e0) * t41544;
    t51188
}
