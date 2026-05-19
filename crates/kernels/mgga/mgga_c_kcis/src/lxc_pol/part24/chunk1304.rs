//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1304/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1304<F: Float>(t100765: F, t100768: F, t100778: F, t100781: F, t100790: F, t101101: F, t101376: F, t7703: F, t93145: F, t93425: F, t93628: F, t96273: F, t96281: F) -> F {
    let t101522 = -F::cast_from(0.6183646701388888889e-4_f64) * t93425 * t101376 - F::cast_from(0.22109259259259259259e-2_f64) * t100765 + F::cast_from(0.66327777777777777776e-2_f64) * t100768 - F::cast_from(0.55273148148148148147e-3_f64) * t93145 - F::cast_from(0.33163888888888888888e-2_f64) * t100778 + F::cast_from(0.44218518518518518516e-2_f64) * t100781 + F::cast_from(0.27802083333333333334e-2_f64) * t7703 * t101101 - F::cast_from(0.11054629629629629629e-2_f64) * t96273 + t93628 + t96281 - F::cast_from(0.22109259259259259259e-2_f64) * t100790;
    t101522
}
