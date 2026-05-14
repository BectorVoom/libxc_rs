//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1159/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1159<F: Float>(t4417: F, t446: F, t7793: F, t93434: F, t29706: F, t432: F, t8411: F, t16261: F, t5674: F, t5675: F, t23054: F, t29642: F, t116418: F, t93355: F, t1564: F, t93409: F) -> (F, F, F, F, F, F, F) {
    let t116477 = t446 * t7793 * t93434 * t4417;
    let t116481 = t446 * t8411 * t29706 * t432;
    let t116485 = t5674 * t8411 * t5675 * t16261;
    let t116487 = t23054 * t29642;
    let t116488 = 2.0 / 9.0 * t116487;
    let t116490 = t5674 * t93355 * t116418;
    let t116493 = t446 * t1564 * t93409 * t4417;
    (t116477, t116481, t116485, t116487, t116488, t116490, t116493)
}
