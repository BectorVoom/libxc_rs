//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 540/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk540<F: Float>(t10324: F, t574: F, t3354: F, t4614: F, t597: F, t2437: F, t2877: F, t2441: F, t8072: F, t895: F, t4752: F, t888: F, t2859: F, t10314: F, t6717: F, t6716: F) -> (F, F, F, F, F, F, F, F) {
    let t10326 = 0.61348681526273199483e1 * t574 * t10324;
    let t10327 = t4614 * t3354;
    let t10329 = 0.15337170381568299871e2 * t597 * t10327;
    let t10331 = 0.35750489951850426669e0 * t2437 * t2877;
    let t10334 = 0.35750489951850426669e0 * t2441 * t2877;
    let t10336 = 0.35750489951850426669e0 * t895 * t8072;
    let t10348 = t4752 * t888;
    let t10350 = 0.7150097990370085334e0 * t2859 * t10348;
    let t10351 = t6717 * t10314;
    let t10353 = 0.69017266717057349418e1 * t6716 * t10351;
    (t10326, t10329, t10331, t10334, t10336, t10348, t10350, t10353)
}
