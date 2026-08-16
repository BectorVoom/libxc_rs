//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 985/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk985<F: Float>(t13942: F, t681: F, t13918: F, t7137: F, t2508: F, t2580: F, t47271: F, t12255: F, t1897: F, t7671: F, t12213: F, t7068: F) -> (F, F, F, F, F) {
    let t47636 = F::cast_from(0.76905262301422242837e-2_f64) * t681 * t13942;
    let t47640 = t7137 * t13918;
    let t47644 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t47271;
    let t47646 = t1897 * t12255 * t7671;
    let t47650 = t1897 * t2580 * t12213 * t7068;
    (t47636, t47640, t47644, t47646, t47650)
}
