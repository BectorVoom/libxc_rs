//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 557/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk557<F: Float>(t50: F, t1289: F, t238: F, t296: F, t5468: F, t5493: F, t5498: F, t822: F, t5492: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t5504 = piecewise3(t51, 0.0, 8.0 / 27.0 * t5493 * t238 + 8.0 / 9.0 * t1289 * t822 - 2.0 / 9.0 * t5498 * t238 + 2.0 / 3.0 * t296 * t5468);
    let t5506 = t5492 / 2.0 + t5504 / 2.0;
    (t5506,)
}
