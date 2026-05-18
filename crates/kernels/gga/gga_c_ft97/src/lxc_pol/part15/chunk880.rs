//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 880/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk880<F: Float>(t2: F, t41446: F, t241: F, t41751: F, t41536: F, t2344: F, t2371: F, t665: F, t7514: F, t675: F, t9567: F, t11176: F, t249: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t42087 = t2 * t41446;
    let t42094 = t41751 * t241;
    let t42095 = t2 * t41536;
    let t42109 = t2344 * t2371;
    let t42110 = t42109 * t2;
    let t42123 = t665 * t7514;
    let t42124 = t42123 * t2;
    let t42163 = t9567 * t675;
    let t42164 = t42163 * t2;
    let t42206 = F::new(280.0) / F::new(81.0) * t11176 * t249;
    (t42087, t42094, t42095, t42109, t42110, t42123, t42124, t42163, t42164, t42206)
}
