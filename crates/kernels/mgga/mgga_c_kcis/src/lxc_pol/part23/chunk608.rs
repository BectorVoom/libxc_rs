//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 608/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk608<F: Float>(t5441: F, t6159: F, t1369: F, t737: F, t1601: F, t167: F, t2105: F, t25: F, t1599: F, t2104: F, t531: F, t833: F, t4440: F, t286: F, t494: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6160 = t6159 * t5441;
    let t6163 = t737 * t1369;
    let t6164 = t1601 * t167;
    let t6165 = t6163 * t6164;
    let t6168 = t25 * t2105;
    let t6169 = t1599 * t6168;
    let t6171 = t2104 * t531;
    let t6172 = t6171 * t833;
    let t6173 = t4440 * t6172;
    let t6176 = t286 * t494;
    (t6160, t6163, t6164, t6165, t6169, t6171, t6172, t6173, t6176)
}
