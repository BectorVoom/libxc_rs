//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1052/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1052<F: Float>(t1017: F, t20027: F, t4462: F, t4668: F, t4454: F, t4714: F, t4458: F, t20023: F, t2102: F, t3499: F, t3506: F, t40437: F, t40466: F, t462: F, t49782: F, t78068: F, t78070: F, t78073: F, t85456: F, t85491: F, t86610: F, t9192: F, t9217: F) -> (F, F, F, F, F, F, F) {
    let t86614 = t20027 * t1017;
    let t86618 = t4462 * t4668;
    let t86622 = t4454 * t4714;
    let t86626 = t4454 * t4668;
    let t86630 = t4458 * t4714;
    let t86637 = t20023 * t1017;
    let t86648 = F::new(2.0) * t462 * t2102 * t86610 - F::new(16.0) / F::new(3.0) * t462 * t9192 * t86614 - F::new(4.0) * t462 * t9217 * t86618 + F::new(4.0) / F::new(3.0) * t462 * t9192 * t86622 - F::new(8.0) / F::new(3.0) * t462 * t40466 * t86626 - F::new(4.0) * t462 * t2102 * t86630 + F::new(8.0) / F::new(3.0) * t462 * t3506 * t85456 + F::new(40.0) / F::new(27.0) * t462 * t40437 * t86637 + F::new(8.0) * t462 * t3499 * t85491 - F::new(8.0) / F::new(9.0) * t78068 + F::new(8.0) / F::new(3.0) * t78070 + F::new(8.0) / F::new(3.0) * t78073 + F::new(112.0) / F::new(81.0) * t49782;
    (t86614, t86618, t86622, t86626, t86630, t86637, t86648)
}
