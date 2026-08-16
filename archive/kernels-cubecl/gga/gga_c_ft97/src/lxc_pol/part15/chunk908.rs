//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 908/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk908<F: Float>(t1526: F, t38308: F, t4406: F, t2178: F, t4790: F, t4657: F, t8232: F, t422: F, t4466: F, t4441: F, t528: F, t1736: F) -> (F, F, F, F, F, F) {
    let t61184 = t1526 * t38308 * t4406;
    let t61366 = t4790 * t2178;
    let t61462 = t8232 * t4657;
    let t61819 = t422 * t4466;
    let t61854 = t4441 * t528;
    let t61866 = t1736 * t4441;
    (t61184, t61366, t61462, t61819, t61854, t61866)
}
