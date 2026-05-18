//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 782/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk782<F: Float>(t13301: F, t1769: F, t9528: F, t2861: F, t5020: F, t5010: F, t5014: F, t1747: F, t3225: F, t2811: F, t4977: F, t2822: F, t5006: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13302 = F::new(0.14739506172839506172e-2) * t13301;
    let t13303 = t9528 * t1769;
    let t13305 = t2861 * t5020;
    let t13307 = t2861 * t5010;
    let t13308 = F::new(0.22109259259259259258e-2) * t13307;
    let t13312 = t2861 * t5014;
    let t13321 = t1747 * t3225;
    let t13322 = t13321 * sigma0;
    let t13376 = t4977 * t2811;
    let t13382 = t2822 * t5006;
    (t13302, t13303, t13305, t13307, t13308, t13312, t13321, t13322, t13376, t13382)
}
