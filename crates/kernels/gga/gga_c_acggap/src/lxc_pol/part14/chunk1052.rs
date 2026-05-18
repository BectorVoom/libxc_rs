//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1052/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1052<F: Float>(t1674: F, t1713: F, t7278: F, t96: F, t9807: F, t1679: F, t1941: F, t2541: F, t104: F, t9805: F, t10586: F, t1954: F, t2254: F, t24893: F, t32241: F, t33352: F, t36592: F, t36601: F, t36605: F, t5645: F, t567: F, t7292: F, t8372: F, t9469: F, t9480: F) -> F {
    let t38589 = t1674 * t7278 * t1713;
    let t38591 = t96 * t9807;
    let t38596 = t1679 * t2541 * t1941;
    let t38603 = t104 * t9805;
    let t38607 = F::new(6.0) * t10586 * t567 * t9469 + F::new(3.0) * t1954 * t38603 * t567 + F::new(6.0) * t2254 * t33352 * t567 - F::new(6.0) * t24893 * t2541 * t8372 + F::new(6.0) * t32241 * t567 * t9469 + F::new(12.0) * t5645 * t7278 * t8372 + F::new(3.0) * t567 * t7292 * t9480 + t36592 - t36601 + t36605 + F::new(6.0) * t38589 + t38591 - t38596;
    t38607
}
