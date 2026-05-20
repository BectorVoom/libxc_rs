//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1344/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1344<F: Float>(t3057: F, t379: F, t1078: F, t1651: F, t3066: F, t1695: F, t3325: F, t3269: F, t3270: F, t11121: F, t5015: F, t999: F) -> (F, F, F, F, F) {
    let t16312 = t3057 * t379;
    let t16313 = t1078 * t1651;
    let t16314 = t16313 * t3066;
    let t16317 = t1695 * t3325;
    let t16318 = t3269 * t16317;
    let t16321 = t1695 * t3270;
    let t16322 = t11121 * t16321;
    let t16327 = t5015 * t999;
    (t16312, t16314, t16318, t16322, t16327)
}
