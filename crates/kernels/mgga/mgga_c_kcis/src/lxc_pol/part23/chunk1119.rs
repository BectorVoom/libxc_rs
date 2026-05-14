//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1119/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1119<F: Float>(t27369: F, t94467: F, t94470: F, t94472: F, t94474: F, t94483: F, t94489: F, t94492: F, t94494: F, t94497: F, t94499: F, t98246: F, t28342: F, t28372: F, t94229: F, t28461: F, t7904: F) -> (F, F, F) {
    let t98507 = 0.46336805555555555556e-3 * t94467 - 0.46336805555555555556e-3 * t94470 - 0.73697530864197530861e-3 * t94472 - 0.22109259259259259258e-2 * t94474 + 0.22109259259259259258e-2 * t94483 - 0.30891203703703703704e-3 * t94489 - 0.30891203703703703704e-3 * t94492 + 0.6183646701388888889e-4 * t94494 + 0.30918233506944444445e-4 * t94497 + 0.23168402777777777778e-3 * t94499 + 0.556528203125e-3 * t27369 * t98246;
    let t98515 = t28372 * t28342 * t94229;
    let t98519 = 0.46336805555555555556e-3 * t28461 * t7904;
    (t98507, t98515, t98519)
}
