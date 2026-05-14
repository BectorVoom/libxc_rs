//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 844/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk844<F: Float>(t13941: F, t2508: F, t779: F, t13945: F, t681: F, t13942: F, t650: F, t270: F, t47420: F, t738: F, t13918: F, t7137: F, t2580: F, t47271: F, t12255: F, t1897: F, t7671: F) -> (F, F, F, F, F, F, F, F) {
    let t47619 = 0.76905262301422242837e-2 * t2508 * t779 * t13941;
    let t47629 = 0.76905262301422242837e-2 * t681 * t13945;
    let t47631 = 0.10254034973522965712e-1 * t650 * t13942;
    let t47634 = 0.76905262301422242837e-2 * t270 * t738 * t47420;
    let t47636 = 0.76905262301422242837e-2 * t681 * t13942;
    let t47640 = t7137 * t13918;
    let t47644 = 0.15381052460284448567e-1 * t2508 * t2580 * t47271;
    let t47646 = t1897 * t12255 * t7671;
    (t47619, t47629, t47631, t47634, t47636, t47640, t47644, t47646)
}
