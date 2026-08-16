//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1255/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1255<F: Float>(t27369: F, t94467: F, t94470: F, t94472: F, t94474: F, t94483: F, t94489: F, t94492: F, t94494: F, t94497: F, t94499: F, t98246: F) -> F {
    let t98507 = F::cast_from(0.46336805555555555556e-3_f64) * t94467 - F::cast_from(0.46336805555555555556e-3_f64) * t94470 - F::cast_from(0.73697530864197530861e-3_f64) * t94472 - F::cast_from(0.22109259259259259258e-2_f64) * t94474 + F::cast_from(0.22109259259259259258e-2_f64) * t94483 - F::cast_from(0.30891203703703703704e-3_f64) * t94489 - F::cast_from(0.30891203703703703704e-3_f64) * t94492 + F::cast_from(0.6183646701388888889e-4_f64) * t94494 + F::cast_from(0.30918233506944444445e-4_f64) * t94497 + F::cast_from(0.23168402777777777778e-3_f64) * t94499 + F::cast_from(0.556528203125e-3_f64) * t27369 * t98246;
    t98507
}
