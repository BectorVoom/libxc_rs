//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 738/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk738<F: Float>(t12997: F, t188: F, t4595: F, t729: F, t108: F, t176: F, t203: F, t4561: F, t6533: F, t4570: F, t6547: F, t1310: F, t3563: F, t1916: F, t4758: F, t1975: F, t4727: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12998 = t188 * t12997;
    let t13002 = t729 * t4595;
    let t13004 = t176 * t13002 * t108;
    let t13005 = t13004 * t203;
    let t13007 = t6533 * t4561;
    let t13020 = t6547 * t4570;
    let t13050 = t1310 * t3563;
    let t13053 = t1916 * t4758;
    let t13054 = t188 * t13053;
    let t13056 = t4727 * t1975;
    (t12998, t13004, t13005, t13007, t13020, t13050, t13053, t13054, t13056)
}
