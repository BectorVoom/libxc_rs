//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 814/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk814<F: Float>(t598: F, t8978: F, t1967: F, t2299: F, t2294: F, t2137: F, t8396: F, t615: F) -> (F, F, F, F, F) {
    let t8979 = t598 * t8978;
    let t8981 = t1967 * t2299;
    let t8983 = t1967 * t2294;
    let t8998 = t2137 * t8396;
    let t9003 = t615 * t8396;
    (t8979, t8981, t8983, t8998, t9003)
}
