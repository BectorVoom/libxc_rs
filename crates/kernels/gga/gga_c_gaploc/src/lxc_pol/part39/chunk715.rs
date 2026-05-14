//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 715/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk715<F: Float>(t13930: F, t13948: F, t12277: F, t977: F, t12849: F, t12858: F, t12864: F, t13005: F, t13232: F, t13234: F, t13237: F, t13239: F, t13245: F, t13763: F, t13764: F, t13837: F, t331: F) -> (F, F) {
    let t13949 = t13930 + t13948;
    let t13951 = t12277 * t977;
    let t13952 = t13949 * t331 - t12849 + t12858 - t12864 - t13005 - t13232 - t13234 - t13237 + 2.0 * t13239 + t13245 - t13763 + t13764 + t13837 - t13951;
    (t13949, t13952)
}
