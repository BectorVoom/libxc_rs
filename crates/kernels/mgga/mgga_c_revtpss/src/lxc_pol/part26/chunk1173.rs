//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1173/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1173<F: Float>(t7058: F, t95644: F, t7064: F, t25387: F, t95571: F, t11050: F, t26497: F, t92975: F, t92942: F, t92944: F, t92946: F, t92948: F, t92952: F, t92956: F, t92958: F, t92960: F, t92963: F, t92966: F, t92969: F, t92971: F, t92973: F) -> (F, F, F, F, F) {
    let t95645 = t7058 * t95644;
    let t95647 = t7064 * t95644;
    let t95649 = t25387 * t95571;
    let t95651 = t26497 * t11050;
    let t95666 = F::new(0.18295201011342718161e-3) * t92975;
    let t95667 = F::new(0.51448821741683684367e-2) * t92942 - F::new(0.51448821741683684367e-1) * t92944 + F::new(0.10289764348336736873e-1) * t92946 + F::new(0.10289764348336736873e-1) * t92948 - F::new(0.96037800584476210818e-1) * t92952 + F::new(0.12196800674228478774e-2) * t92956 + F::new(0.10289764348336736873e-1) * t92958 - F::new(0.25724410870841842183e-2) * t92960 + F::new(0.30492001685571196935e-4) * t92963 - F::new(0.2168591159877823526e-3) * t92966 - F::new(35.0) / F::new(36.0) * t92969 + F::new(7.0) / F::new(24.0) * t92971 - t92973 / F::new(24.0) + t95666;
    (t95645, t95647, t95649, t95651, t95667)
}
