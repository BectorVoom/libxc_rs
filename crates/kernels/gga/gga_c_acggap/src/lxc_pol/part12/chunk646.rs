//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 646/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk646<F: Float>(t1487: F, t301: F, t1089: F, t368: F, t372: F, t1083: F, t398: F, t1539: F, t360: F, t1181: F, t1532: F, t1163: F) -> (F, F, F, F, F, F, F) {
    let t5111 = t1487 * t301;
    let t5113 = t1089 * t368 * t5111;
    let t5116 = t1487 * t372;
    let t5118 = t398 * t1083 * t5116;
    let t5122 = t1539 * t360;
    let t5124 = t1181 * t1532 * t5122;
    let t5126 = F::new(0.85748036236139473944e-3) * t1163 * t5124;
    (t5111, t5113, t5116, t5118, t5122, t5124, t5126)
}
