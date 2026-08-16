//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 963/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk963<F: Float>(t1246: F, t159: F, t210: F, t3234: F, t520: F, t1212: F, t2139: F, t1215: F, t242: F, t527: F, t8200: F, t525: F) -> (F, F, F, F, F, F) {
    let t10140 = t159 * t1246;
    let t10141 = t210 * t10140;
    let t10151 = t520 * t3234;
    let t10160 = t2139 * t1212;
    let t10161 = t10160 * t1215;
    let t10164 = t8200 * t527 * t242;
    let t10166 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t525 * t10164;
    (t10141, t10151, t10160, t10161, t10164, t10166)
}
