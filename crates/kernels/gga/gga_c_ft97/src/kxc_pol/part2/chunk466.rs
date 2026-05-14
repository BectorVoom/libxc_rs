//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 466/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk466<F: Float>(t2253: F, t895: F, t906: F, t70: F, t703: F, t2347: F, t327: F, t2349: F, t230: F, t900: F) -> (F, F, F, F, F) {
    let t2913 = t2253 * t895;
    let t2915 = t2253 * t906;
    let t2917 = t70 * t703;
    let t2918 = t327 * t2347;
    let t2920 = t2917 * t2918 * t2349;
    let t2923 = t230 * t900;
    (t2913, t2915, t2917, t2920, t2923)
}
