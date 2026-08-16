//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1322/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1322(t22849: f64, t7252: f64, t22877: f64, t94516: f64, t22881: f64, t26028: f64, t22895: f64, t22837: f64, t22843: f64, t27940: f64, t22833: f64, t108566: f64, t108570: f64, t108576: f64, t94472: f64, t94474: f64, t98174: f64, t98200: f64) -> f64 {
    let t114541 = t7252 * t22849;
    let t114543 = t94516 * t22877;
    let t114545 = t26028 * t22881;
    let t114547 = t26028 * t22895;
    let t114549 = t26028 * t22837;
    let t114551 = t27940 * t22843;
    let t114553 = t27940 * t22833;
    let t114556 = 0.81312004494856525162e-3_f64 * t98174 - 0.76230004213927992339e-4_f64 * t108566 - 0.76230004213927992339e-4_f64 * t108570 + 0.60023625365297631762e-2_f64 * t108576 - t114541 / 48.0_f64 - t114543 / 4.0_f64 - t94472 - 0.12862205435420921092e-2_f64 * t114545 - 0.25724410870841842184e-1_f64 * t114547 + 0.51448821741683684367e-2_f64 * t114549 - 0.10289764348336736873e-1_f64 * t114551 + 0.25724410870841842183e-2_f64 * t114553 + 0.60984003371142393869e-4_f64 * t98200 + t94474;
    t114556
}
