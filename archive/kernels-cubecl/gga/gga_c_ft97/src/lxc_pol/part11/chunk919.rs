//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 919/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk919<F: Float>(t8392: F, t8426: F, t492: F, t7765: F, t1559: F, t1588: F, t432: F, t1636: F, t443: F, t444: F) -> (F, F, F, F, F) {
    let t38935 = t8392 * t8426;
    let t38937 = t7765 * t492;
    let t38942 = t1559 * t1588;
    let t38947 = t7765 * t432;
    let t38953 = t443 * t444 * t1636;
    (t38935, t38937, t38942, t38947, t38953)
}
