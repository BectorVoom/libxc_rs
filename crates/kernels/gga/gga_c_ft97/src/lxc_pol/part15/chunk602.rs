//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 602/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk602<F: Float>(t1526: F, t4641: F, t7705: F, t142: F, t8633: F, t2258: F, t342: F, t4645: F, t630: F, t1882: F, t4657: F, t4668: F, t7368: F, t1546: F, t4664: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t16631 = t1526 * t7705 * t4641;
    let t16633 = t8633 * t142;
    let t16640 = t2258 * t142;
    let t16649 = t342 * t630 * t4645;
    let t16679 = t1882 * t4657;
    let t16736 = t7368 * t4668;
    let t16745 = t89 * t1546 * t4664;
    (t16631, t16633, t16640, t16649, t16679, t16736, t16745)
}
