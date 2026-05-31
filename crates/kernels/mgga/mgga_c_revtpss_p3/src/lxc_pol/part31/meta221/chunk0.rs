//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 992/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk992<F: Float>(t5826: F, t70: F, t1470: F, t1486: F, t2275: F, t5819: F, t48: F, t5825: F, t476: F, t53: F, t2282: F, t60: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t5827 = t5826 * t70;
    let t5830 = t1470 * t1486;
    let t5835 = t2275 * t5819;
    let t5838 = t48 * t5825;
    let t5842 = F::cast_from(1.0_f64) / t53 / t476;
    let t5843 = sigma2 * t5842;
    let t5848 = t2282 * t5819;
    let t5851 = t60 * t5825;
    (t5827, t5830, t5835, t5838, t5843, t5848, t5851)
}
