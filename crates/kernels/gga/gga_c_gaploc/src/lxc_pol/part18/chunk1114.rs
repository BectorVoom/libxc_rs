//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1114/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1114<F: Float>(t32435: F, t688: F, t2508: F, t779: F, t10816: F, t32163: F, t5836: F, t29492: F, t29494: F, t29498: F, t29501: F, t29503: F, t32639: F, t32642: F, t32644: F, t32646: F, t32650: F, t32653: F, t32657: F) -> (F,) {
    let t32658 = t32435 * t688;
    let t32661 = 0.15381052460284448567e-1 * t2508 * t779 * t32658;
    let t32664 = 0.10766736722199113997e0 * t32163 * t10816 * t5836;
    let t32665 = -t32639 + t32642 + t32644 + t32646 + t32650 - t32653 + t32657 + t32661 + t29492 + t29494 + t29498 - t29501 + t32664 + t29503;
    (t32665,)
}
