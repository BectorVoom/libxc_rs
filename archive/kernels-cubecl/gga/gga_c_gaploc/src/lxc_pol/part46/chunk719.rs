//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 719/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk719<F: Float>(t13048: F, t13075: F, t13123: F, t13164: F, t13101: F, t738: F, t13096: F, t169: F, t299: F, t706: F, t2558: F, t3464: F) -> (F, F, F, F, F) {
    let t13166 = t13048 + t13075 + t13123 + t13164;
    let t13168 = t738 * t13101;
    let t13172 = t13096 * t169 * t299;
    let t13173 = t706 * t13172;
    let t13176 = t3464 * t2558;
    (t13166, t13168, t13172, t13173, t13176)
}
