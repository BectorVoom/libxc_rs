//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 892/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk892<F: Float>(t487: F, t8460: F, t492: F, t1825: F, t8355: F, t422: F, t626: F, t1526: F, t1529: F, t1565: F, t7705: F, t7725: F) -> (F, F, F, F, F, F) {
    let t38299 = t8460 * t487;
    let t38300 = t38299 * t492;
    let t38304 = t1825 * t8355;
    let t38308 = t626 * t422;
    let t38310 = t1526 * t38308 * t1529;
    let t38313 = t1526 * t7705 * t1565;
    let t38316 = t1526 * t7705 * t7725;
    (t38300, t38304, t38308, t38310, t38313, t38316)
}
