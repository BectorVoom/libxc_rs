//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 639/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk639<F: Float>(t1882: F, t4553: F, t4569: F, t4595: F, t4844: F, t5: F, t1526: F, t4641: F, t7705: F, t142: F, t8633: F, t2258: F) -> (F, F, F, F, F, F, F) {
    let t16490 = t1882 * t4553;
    let t16539 = t1882 * t4569;
    let t16541 = t1882 * t4595;
    let t16612 = t5 * t4844;
    let t16631 = t1526 * t7705 * t4641;
    let t16633 = t8633 * t142;
    let t16640 = t2258 * t142;
    (t16490, t16539, t16541, t16612, t16631, t16633, t16640)
}
