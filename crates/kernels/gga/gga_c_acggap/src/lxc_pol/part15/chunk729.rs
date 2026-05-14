//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 729/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk729<F: Float>(t1089: F, t2090: F, t4643: F, t598: F, t1967: F, t2299: F, t2294: F, t2137: F, t8396: F) -> (F, F, F, F, F) {
    let t8978 = t1089 * t4643 * t2090;
    let t8979 = t598 * t8978;
    let t8981 = t1967 * t2299;
    let t8983 = t1967 * t2294;
    let t8998 = t2137 * t8396;
    (t8978, t8979, t8981, t8983, t8998)
}
