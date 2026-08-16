//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1255/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1255(t27369: f64, t94467: f64, t94470: f64, t94472: f64, t94474: f64, t94483: f64, t94489: f64, t94492: f64, t94494: f64, t94497: f64, t94499: f64, t98246: f64) -> f64 {
    let t98507 = 0.46336805555555555556e-3_f64 * t94467 - 0.46336805555555555556e-3_f64 * t94470 - 0.73697530864197530861e-3_f64 * t94472 - 0.22109259259259259258e-2_f64 * t94474 + 0.22109259259259259258e-2_f64 * t94483 - 0.30891203703703703704e-3_f64 * t94489 - 0.30891203703703703704e-3_f64 * t94492 + 0.6183646701388888889e-4_f64 * t94494 + 0.30918233506944444445e-4_f64 * t94497 + 0.23168402777777777778e-3_f64 * t94499 + 0.556528203125e-3_f64 * t27369 * t98246;
    t98507
}
