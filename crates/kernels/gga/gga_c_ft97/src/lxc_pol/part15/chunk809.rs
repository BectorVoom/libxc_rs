//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 809/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk809<F: Float>(t38953: F, t4608: F, t4561: F, t8232: F, t4557: F, t1851: F, t4545: F, t2252: F, t342: F, t4410: F, t1526: F, t38308: F, t4406: F, t2178: F, t4790: F, t4657: F) -> (F, F, F, F, F, F, F, F) {
    let t60756 = t38953 * t4608;
    let t60919 = t8232 * t4561;
    let t60984 = t8232 * t4557;
    let t61025 = t4545 * t1851;
    let t61180 = t342 * t2252 * t4410;
    let t61184 = t1526 * t38308 * t4406;
    let t61366 = t4790 * t2178;
    let t61462 = t8232 * t4657;
    (t60756, t60919, t60984, t61025, t61180, t61184, t61366, t61462)
}
