//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1248/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1248<F: Float>(t2508: F, t29194: F, t2936: F, t1890: F, t21446: F, t9014: F, t32435: F, t688: F, t779: F, t10816: F, t32163: F, t5836: F) -> (F, F, F, F) {
    let t32653 = F::new(0.10766736722199113997e0) * t2508 * t2936 * t29194;
    let t32657 = F::new(0.1845726295234133828e0) * t2508 * t9014 * t1890 * t21446;
    let t32658 = t32435 * t688;
    let t32661 = F::new(0.15381052460284448567e-1) * t2508 * t779 * t32658;
    let t32664 = F::new(0.10766736722199113997e0) * t32163 * t10816 * t5836;
    (t32653, t32657, t32661, t32664)
}
