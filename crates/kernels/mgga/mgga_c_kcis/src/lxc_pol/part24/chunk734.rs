//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 734/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk734<F: Float>(t13307: F, t2861: F, t5014: F, t1747: F, t3225: F, t2811: F, t4977: F, t2822: F, t5006: F, t5000: F, t251: F, t691: F, t1018: F, t86: F, t4996: F, t4989: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13308 = 0.22109259259259259258e-2 * t13307;
    let t13312 = t2861 * t5014;
    let t13321 = t1747 * t3225;
    let t13322 = t13321 * sigma0;
    let t13376 = t4977 * t2811;
    let t13382 = t2822 * t5006;
    let t13391 = t2822 * t5000;
    let t13396 = t691 * t251;
    let t13398 = t86 * t13396 * t1018;
    let t13399 = t13398 * t4996;
    let t13408 = t2822 * t4989;
    (t13308, t13312, t13321, t13322, t13376, t13382, t13391, t13398, t13399, t13408)
}
