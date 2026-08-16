//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 858/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk858<F: Float>(t723: F, t8469: F, t1445: F, t1710: F, t2958: F, t2936: F, t769: F, t2089: F, t2925: F, t3009: F, t1457: F, t1022: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8470 = t8469 * t723;
    let t8471 = t1445 * t8470;
    let t8474 = t2958 * t1710;
    let t8475 = t1445 * t8474;
    let t8478 = t769 * t2936;
    let t8483 = t2089 * t2925;
    let t8484 = t8483 * t723;
    let t8485 = t1445 * t8484;
    let t8488 = t3009 * t1710;
    let t8489 = t1445 * t8488;
    let t8494 = t1457 * t8470;
    let t8497 = t1457 * t8474;
    let t8502 = t1022 * t723;
    (t8470, t8471, t8474, t8475, t8478, t8483, t8485, t8489, t8494, t8497, t8502)
}
