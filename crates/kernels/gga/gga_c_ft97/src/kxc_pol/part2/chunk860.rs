//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 860/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk860<F: Float>(t13373: F, t2372: F, t713: F, t1934: F, t3712: F, t2493: F, t1131: F, t2347: F, t2349: F, t9916: F, t1775: F, t3927: F) -> (F, F, F, F, F, F) {
    let t13375 = t2372 * t13373 * t713;
    let t13378 = t3712 * t1934;
    let t13379 = t2493 * t13378;
    let t13382 = t1131 * t2347;
    let t13383 = t13382 * t2349;
    let t13384 = t9916 * t13383;
    let t13388 = F::new(2.0) / F::new(9.0) * t1775 * t3927;
    (t13375, t13378, t13379, t13383, t13384, t13388)
}
