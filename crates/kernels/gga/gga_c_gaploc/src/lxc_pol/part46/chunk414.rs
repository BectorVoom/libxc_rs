//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 414/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk414<F: Float>(t2102: F, t769: F, t325: F, t539: F, t107: F, t2086: F, t801: F) -> (F, F, F, F) {
    let t5771 = t769 * t2102;
    let t5774 = t539 * t325;
    let t5775 = t107 * t5774;
    let t5782 = t801 * t2086;
    (t5771, t5774, t5775, t5782)
}
