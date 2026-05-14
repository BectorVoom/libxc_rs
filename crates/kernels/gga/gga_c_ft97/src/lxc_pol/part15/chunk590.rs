//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 590/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk590<F: Float>(t1170: F, t8232: F, t1263: F, t8640: F, t1270: F, t2440: F, t327: F, t10845: F, t2347: F, t2360: F, t2923: F, t10864: F, t1268: F, t1186: F, t89: F, t9733: F) -> (F, F, F, F, F, F, F, F) {
    let t14233 = t8232 * t1170;
    let t14431 = t8640 * t1263;
    let t14445 = t8640 * t1270;
    let t14487 = t2440 * t327;
    let t14514 = t10845 * t2347;
    let t14519 = t2923 * t2360;
    let t14523 = t10864 * t1268;
    let t14715 = t89 * t9733 * t1186;
    (t14233, t14431, t14445, t14487, t14514, t14519, t14523, t14715)
}
