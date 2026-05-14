//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 858/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk858<F: Float>(t143263: F, t141365: F, t7638: F, t7642: F, t33288: F, t33811: F, t33813: F, t2360: F, t7611: F, t2680: F, t33953: F, t33967: F, t681: F, t89: F, t33971: F, t33984: F) -> (F, F, F, F, F, F, F, F, F) {
    let t143264 = 8.0 / 27.0 * t143263;
    let t143273 = t7638 * t141365 * t7642;
    let t143274 = 10.0 / 27.0 * t143273;
    let t143276 = t33811 * t33288 * t33813;
    let t143284 = t7611 * t2360;
    let t143293 = t2680 * t33953;
    let t143321 = t89 * t681 * t33967;
    let t143324 = t89 * t681 * t33971;
    let t143327 = t89 * t681 * t33984;
    (t143264, t143273, t143274, t143276, t143284, t143293, t143321, t143324, t143327)
}
