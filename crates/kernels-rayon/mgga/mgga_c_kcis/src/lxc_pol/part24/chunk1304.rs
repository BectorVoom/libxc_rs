//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1304/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1304(t100765: f64, t100768: f64, t100778: f64, t100781: f64, t100790: f64, t101101: f64, t101376: f64, t7703: f64, t93145: f64, t93425: f64, t93628: f64, t96273: f64, t96281: f64) -> f64 {
    let t101522 = -0.6183646701388888889e-4_f64 * t93425 * t101376 - 0.22109259259259259259e-2_f64 * t100765 + 0.66327777777777777776e-2_f64 * t100768 - 0.55273148148148148147e-3_f64 * t93145 - 0.33163888888888888888e-2_f64 * t100778 + 0.44218518518518518516e-2_f64 * t100781 + 0.27802083333333333334e-2_f64 * t7703 * t101101 - 0.11054629629629629629e-2_f64 * t96273 + t93628 + t96281 - 0.22109259259259259259e-2_f64 * t100790;
    t101522
}
