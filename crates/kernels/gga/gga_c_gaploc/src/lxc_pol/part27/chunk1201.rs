//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1201/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1201<F: Float>(t2508: F, t2580: F, t32223: F, t21556: F, t3420: F, t10773: F, t7137: F, t3448: F, t24487: F, t948: F, t2586: F, t8637: F) -> (F, F, F, F, F, F) {
    let t32226 = F::new(0.15381052460284448567e-1) * t2508 * t2580 * t32223;
    let t32241 = F::new(0.20508069947045931424e-1) * t21556 * t3420;
    let t32243 = F::new(0.20508069947045931424e-1) * t7137 * t10773;
    let t32245 = F::new(0.41016139894091862846e-1) * t21556 * t3448;
    let t32253 = F::new(0.23071578690426672851e-1) * t2508 * t24487 * t948;
    let t32256 = F::new(0.46143157380853345702e-1) * t2508 * t8637 * t2586;
    (t32226, t32241, t32243, t32245, t32253, t32256)
}
