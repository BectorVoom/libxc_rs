//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 834/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk834<F: Float>(t213: F, t37481: F, t7464: F, t36835: F, t5567: F, t109230: F, t1613: F, t92354: F, t9533: F, t218: F, t41: F, t2344: F, t679: F, t7205: F, t33432: F, t3789: F, t7203: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t141107 = t37481 * t213 * t7464;
    let t141111 = t36835 * t5567;
    let t141112 = t109230 * t141111;
    let t141116 = t92354 * t1613 * sigma2;
    let t141117 = t9533 * t141116;
    let t141121 = t1613 * t218;
    let t141123 = t9533 * t41 * t141121;
    let t141125 = t7205 * t2344 * t679;
    let t141160 = t3789 * t33432 * t7203;
    (t141107, t141111, t141112, t141116, t141117, t141123, t141125, t141160)
}
