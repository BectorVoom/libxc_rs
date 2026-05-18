//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1322/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1322<F: Float>(t22849: F, t7252: F, t22877: F, t94516: F, t22881: F, t26028: F, t22895: F, t22837: F, t22843: F, t27940: F, t22833: F, t108566: F, t108570: F, t108576: F, t94472: F, t94474: F, t98174: F, t98200: F) -> F {
    let t114541 = t7252 * t22849;
    let t114543 = t94516 * t22877;
    let t114545 = t26028 * t22881;
    let t114547 = t26028 * t22895;
    let t114549 = t26028 * t22837;
    let t114551 = t27940 * t22843;
    let t114553 = t27940 * t22833;
    let t114556 = F::new(0.81312004494856525162e-3) * t98174 - F::new(0.76230004213927992339e-4) * t108566 - F::new(0.76230004213927992339e-4) * t108570 + F::new(0.60023625365297631762e-2) * t108576 - t114541 / F::new(48.0) - t114543 / F::new(4.0) - t94472 - F::new(0.12862205435420921092e-2) * t114545 - F::new(0.25724410870841842184e-1) * t114547 + F::new(0.51448821741683684367e-2) * t114549 - F::new(0.10289764348336736873e-1) * t114551 + F::new(0.25724410870841842183e-2) * t114553 + F::new(0.60984003371142393869e-4) * t98200 + t94474;
    t114556
}
