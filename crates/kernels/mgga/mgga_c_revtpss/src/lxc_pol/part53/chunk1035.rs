//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1035/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1035<F: Float>(t31844: F, t826: F, t126046: F, t247: F, t31752: F, t4366: F, t126250: F, t31805: F, t31801: F, t119941: F, t119969: F, t119976: F, t119983: F, t119985: F, t119990: F, t119991: F, t119995: F, t126246: F, t126252: F, t126256: F, t126260: F, t126271: F, t126273: F, t1955: F, t27207: F, t31794: F, t31812: F, t7048: F, t7759: F, t7769: F, t7779: F, t8481: F, t8649: F, t8650: F) -> (F,) {
    let t126276 = t31844 * t826;
    let t126280 = t31752 * t126276 * t247 * t126046 * t4366;
    let t126282 = t31805 * t126250;
    let t126283 = t126282 * t31801;
    let t126290 = 0.17347256376410398924e1 * t119941 * t27207 + 0.8673628188205199462e0 * t31794 * t126246 + 0.18822977838986977999e-4 * t119969 - 0.14279934416275588154e-1 * t126252 + t119976 + 0.56468933516960934e-2 * t126256 + 0.56468933516960933998e-3 * t126260 + 0.11423947533020470523e1 * t8649 * t8650 * t7048 * t7759 + 0.42839803248826764462e-1 * t119983 - 0.28559868832551176308e-1 * t119985 - 0.17347256376410398924e1 * t1955 * t7048 * t7779 + 0.50779446784275991476e-1 * t126271 + 0.57119737665102352616e0 * t126273 * t8481 + 0.56468933516960933998e-3 * t126280 + 0.25389723392137995738e-1 * t126283 - 0.3427184259906141157e1 * t8649 * t31812 * t7769 * t7048 + t119990 + 0.75291911355947911996e-4 * t119991 + t119995;
    (t126290,)
}
