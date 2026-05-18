//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 792/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk792<F: Float>(t13847: F, t2685: F, t2684: F, t969: F, t825: F, t13010: F, t13015: F, t13018: F, t13021: F, t13026: F, t13028: F, t13029: F, t13031: F, t13036: F, t13040: F, t13044: F, t13047: F) -> (F, F, F) {
    let t13848 = t2685 * t13847;
    let t13849 = t2684 * t13848;
    let t13851 = t969 * t13847;
    let t13852 = t825 * t13851;
    let t13854 = -F::new(0.69017266717057349418e1) * t13010 - t13015 - t13018 + F::new(0.71500979903700853338e0) * t13021 + t13026 + t13028 + F::new(0.35750489951850426669e0) * t13029 - F::new(0.10725146985555128001e1) * t13031 + t13036 - t13040 + t13044 - t13047 + F::new(0.19171462976960374838e0) * t13849 - F::new(0.19171462976960374838e0) * t13852;
    (t13848, t13851, t13854)
}
