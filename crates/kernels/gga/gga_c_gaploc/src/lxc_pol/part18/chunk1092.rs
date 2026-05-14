//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1092/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1092<F: Float>(t10722: F, t10776: F, t10790: F, t11083: F, t1841: F, t1897: F, t2042: F, t2508: F, t2580: F, t32207: F, t32213: F, t32215: F, t32222: F, t32226: F, t32230: F, t32234: F, t32241: F, t32243: F, t32245: F, t3433: F, t5288: F, t5293: F, t702: F, t7129: F, t7226: F, t7289: F) -> (F,) {
    let t32248 = 0.76905262301422242837e-2 * t2508 * t2042 * t3433 + t32207 + 0.20508069947045931424e-1 * t5293 * t10722 + t32213 - 0.34180116578409885705e-2 * t1841 * t7289 * t32215 + t32222 + t32226 - 0.15381052460284448567e-1 * t1897 * t11083 * t702 + 0.92286314761706691403e-1 * t2508 * t2580 * t32230 - 0.46143157380853345701e-1 * t2508 * t7226 * t32234 - 0.15381052460284448567e-1 * t5288 * t10776 + t32241 + t32243 + t32245 - 0.46143157380853345702e-1 * t7129 * t10790;
    (t32248,)
}
