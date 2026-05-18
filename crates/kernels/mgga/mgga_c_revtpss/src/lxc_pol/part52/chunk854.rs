//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 854/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk854<F: Float>(t3123: F, t7121: F, t365: F, t3089: F, t1087: F, t1024: F, t7131: F, t3167: F, t7120: F, t1033: F, t3173: F, t7122: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t25512 = t3123 * t7121;
    let t25515 = sigma0 * t365;
    let t25516 = t25515 * t3089;
    let t25517 = t1087 * t25516;
    let t25522 = t1024 * t7131;
    let t25525 = t7120 * t3167;
    let t25526 = t1033 * t25525;
    let t25529 = t7122 * t3173;
    (t25512, t25515, t25516, t25517, t25522, t25526, t25529)
}
