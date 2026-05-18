//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 784/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk784<F: Float>(t598: F, t8497: F, t1083: F, t1089: F, t8484: F, t8489: F, t7458: F, t1980: F, t1988: F, t2299: F, t1530: F, t7646: F) -> (F, F, F, F, F, F, F, F) {
    let t8498 = t598 * t8497;
    let t8502 = t1089 * t1083 * t8484;
    let t8503 = t598 * t8502;
    let t8505 = t1083 * t8489;
    let t8506 = t7458 * t8505;
    let t8507 = t1980 * t8506;
    let t8509 = t1988 * t2299;
    let t8511 = t1530 * t7646;
    (t8498, t8502, t8503, t8505, t8506, t8507, t8509, t8511)
}
