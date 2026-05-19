//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1157/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1157<F: Float>(t13918: F, t7137: F, t2508: F, t2580: F, t47271: F, t12255: F, t1897: F, t7671: F, t12213: F, t7068: F, t13934: F, t731: F) -> (F, F, F, F, F) {
    let t47640 = t7137 * t13918;
    let t47644 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t47271;
    let t47646 = t1897 * t12255 * t7671;
    let t47650 = t1897 * t2580 * t12213 * t7068;
    let t47652 = t731 * t13934;
    (t47640, t47644, t47646, t47650, t47652)
}
