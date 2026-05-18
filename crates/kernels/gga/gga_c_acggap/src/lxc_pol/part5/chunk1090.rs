//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1090/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1090<F: Float>(t14947: F, t1674: F, t19441: F, t19444: F, t19451: F, t19452: F, t19453: F, t19454: F, t19455: F, t19456: F, t19510: F, t291: F, t301: F, t6610: F, t6621: F, t839: F, t96: F) -> F {
    let t19514 = F::new(12.0) * t1674 * t19444 * t301 + F::new(6.0) * t1674 * t6610 * t839 + F::new(3.0) * t19510 * t291 * t96 + F::new(12.0) * t14947 * t6621 - t19441 - t19451 + t19452 + t19453 + t19454 - t19455 + t19456;
    t19514
}
