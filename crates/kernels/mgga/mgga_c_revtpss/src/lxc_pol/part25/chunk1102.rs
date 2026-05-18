//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1102/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1102<F: Float>(t136: F, t243: F, t3133: F, t3302: F, t357: F, t2371: F, t94: F, t4982: F, t999: F, t2007: F, t197: F, t531: F) -> (F, F, F, F, F, F, F, F) {
    let t14685 = t243 * t136;
    let t16573 = t3302 * t3133 * t357;
    let t18163 = t94 * t2371;
    let t19482 = t3302 * t357;
    let t19502 = t4982 * t999;
    let t19579 = t19482 * t999;
    let t25078 = t2007 * t2371;
    let t25081 = t197 * t531;
    (t14685, t16573, t18163, t19482, t19502, t19579, t25078, t25081)
}
