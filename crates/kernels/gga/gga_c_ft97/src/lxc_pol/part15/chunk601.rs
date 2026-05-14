//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 601/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk601<F: Float>(t2: F, t4436: F, t4531: F, t458: F, t4527: F, t1775: F, t4519: F, t4523: F, t4512: F, t4505: F, t8345: F, t1882: F, t4553: F, t4569: F, t4595: F, t4844: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16399 = t2 * t4436;
    let t16404 = t458 * t4531;
    let t16406 = t458 * t4527;
    let t16442 = t1775 * t4519;
    let t16444 = t1775 * t4523;
    let t16446 = t1775 * t4512;
    let t16474 = t8345 * t4505;
    let t16490 = t1882 * t4553;
    let t16539 = t1882 * t4569;
    let t16541 = t1882 * t4595;
    let t16612 = t5 * t4844;
    (t16399, t16404, t16406, t16442, t16444, t16446, t16474, t16490, t16539, t16541, t16612)
}
