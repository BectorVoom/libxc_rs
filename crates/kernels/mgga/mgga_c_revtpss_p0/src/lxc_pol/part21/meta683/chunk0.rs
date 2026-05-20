//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2497/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2497<F: Float>(t3718: F, t3722: F, t44546: F, t3566: F, t3766: F, t5330: F, t12646: F, t12915: F, t247: F, t5384: F, t12831: F, t12865: F) -> (F, F, F, F) {
    let t44548 = t3718 * t44546 * t3722;
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44559 = t5384 * t247 * t12915 * t12646;
    let t44561 = t12831 * t12865;
    (t44548, t44551, t44559, t44561)
}
