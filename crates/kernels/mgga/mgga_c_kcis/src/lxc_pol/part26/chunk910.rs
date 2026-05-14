//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 910/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk910<F: Float>(t22509: F, t22542: F, t22581: F, t22627: F, t552: F, t573: F, t12565: F, t7393: F, t21791: F, t577: F, t585: F, t20956: F, t4293: F, t4292: F, t2062: F, t6020: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t22629 = t22509 + t22542 + t22581 + t22627;
    let t22630 = t22629 * t552;
    let t22631 = t22630 * sigma2;
    let t22632 = t22631 * t573;
    let t22634 = t12565 * t7393;
    let t22636 = t21791 * t552;
    let t22637 = t22636 * t577;
    let t22638 = t22637 * t585;
    let t22640 = t4293 * t20956;
    let t22641 = t4292 * t22640;
    let t22643 = t6020 * t2062;
    (t22630, t22632, t22634, t22636, t22638, t22640, t22641, t22643)
}
