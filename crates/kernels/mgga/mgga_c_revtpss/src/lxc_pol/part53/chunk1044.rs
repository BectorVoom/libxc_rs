//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1044/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1044<F: Float>(t32299: F, t32301: F, t32303: F, t32305: F, t32307: F, t32309: F, t32312: F, t32320: F, t32323: F, t32325: F, t32329: F, t32338: F, t32340: F, t6985: F, t7591: F) -> F {
    let t32883 = -F::new(2.0) * t6985 * t7591 + t32299 - F::new(2.0) * t32301 - F::new(2.0) * t32303 - F::new(2.0) * t32305 - F::new(2.0) * t32307 - F::new(2.0) * t32309 - F::new(2.0) * t32312 - t32320 + t32323 - F::new(2.0) * t32325 + t32329 - t32338 - t32340;
    t32883
}
