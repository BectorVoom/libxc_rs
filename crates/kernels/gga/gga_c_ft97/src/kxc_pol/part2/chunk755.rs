//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 755/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk755<F: Float>(t13378: F, t2493: F, t1131: F, t2347: F, t2349: F, t9916: F, t1775: F, t3927: F, t3821: F, t668: F, t505: F, t2380: F, t2393: F, t200: F, t1609: F, t213: F) -> (F, F, F, F, F, F, F, F) {
    let t13379 = t2493 * t13378;
    let t13382 = t1131 * t2347;
    let t13383 = t13382 * t2349;
    let t13384 = t9916 * t13383;
    let t13388 = 2.0 / 9.0 * t1775 * t3927;
    let t13389 = t3821 * t668;
    let t13390 = t13389 * t505;
    let t13391 = t2493 * t13390;
    let t13394 = t2393 * t2380;
    let t13395 = t13394 * t200;
    let t13399 = t1609 * t213;
    (t13379, t13383, t13384, t13388, t13390, t13391, t13395, t13399)
}
