//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1013/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1013<F: Float>(t11273: F, t3160: F, t2923: F, t910: F, t287: F, t2922: F, t275: F, t11132: F, t240: F, t624: F, t281: F, t283: F) -> (F, F, F, F, F, F, F) {
    let t11277 = t11273 * t3160;
    let t11294 = t910 * t2923;
    let t11298 = F::new(1.0) / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = F::new(28.0) / F::new(27.0) * t11132;
    let t11334 = F::new(0.93011851851851851854e0) * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    (t11277, t11294, t11299, t11304, t11334, t11335, t11337)
}
