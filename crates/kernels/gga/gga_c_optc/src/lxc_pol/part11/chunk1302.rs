//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1302/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1302<F: Float>(t265: F, t57197: F, t57209: F, t241: F, t1343: F, t50721: F, t14267: F, t4856: F, t10493: F, t16674: F, t1342: F, t16858: F, t2373: F) -> (F, F, F, F, F, F) {
    let t57211 = (t57197 + t57209) * t265;
    let t57213 = F::new(0.19751789702565206229e-1) * t241 * t57211;
    let t57215 = F::new(4.0) * t50721 * t1343;
    let t57217 = F::new(0.70178680769462448852e1) * t14267 * t4856;
    let t57219 = F::new(0.19298189186581325787e3) * t10493 * t16674;
    let t57222 = F::new(8.0) * t2373 * t16858 * t1342;
    (t57211, t57213, t57215, t57217, t57219, t57222)
}
