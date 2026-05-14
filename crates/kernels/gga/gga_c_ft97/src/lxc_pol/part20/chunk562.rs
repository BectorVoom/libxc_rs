//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 562/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk562<F: Float>(t191: F, t7640: F, t2683: F, t375: F, t89: F, t793: F, t9733: F, t2336: F, t2675: F, t2661: F, t9725: F, t272: F, t9606: F, t2417: F, t274: F, t668: F) -> (F, F, F, F, F, F, F, F) {
    let t10261 = t191 * t7640;
    let t10276 = t89 * t375 * t2683;
    let t10279 = t89 * t9733 * t793;
    let t10282 = t89 * t2336 * t2675;
    let t10286 = t89 * t9725 * t2661;
    let t10304 = 1.0 / t272 / t9606;
    let t10309 = t274 * t2417;
    let t10327 = t274 * t668;
    (t10261, t10276, t10279, t10282, t10286, t10304, t10309, t10327)
}
