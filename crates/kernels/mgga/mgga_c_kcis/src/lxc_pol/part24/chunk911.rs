//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 911/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk911<F: Float>(t26474: F, t26477: F, t2398: F, t2725: F, t7639: F, t7636: F, t7642: F, t7647: F, t209: F, t2746: F, t7645: F, t2155: F, t2751: F, t7637: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26478 = t26474 * t26477;
    let t26480 = t2725 * t2398;
    let t26481 = t26480 * t7639;
    let t26483 = t7636 * t26477;
    let t26485 = t7642 * t7647;
    let t26487 = t7642 * t7639;
    let t26490 = t209 * t7645 * t2746;
    let t26491 = t2155 * t26490;
    let t26494 = t209 * t7637 * t2751;
    (t26478, t26480, t26481, t26483, t26485, t26487, t26490, t26491, t26494)
}
