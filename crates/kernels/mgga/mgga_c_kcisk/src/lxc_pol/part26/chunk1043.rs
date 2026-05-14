//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1043/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1043<F: Float>(t21524: F, t27726: F, t27720: F, t5675: F, t21530: F, t14940: F, t1572: F, t1580: F, t1583: F, t21500: F, t21511: F, t21515: F, t21519: F, t21537: F, t21555: F, t21558: F, t21567: F, t27702: F, t27706: F, t27710: F, t27722: F, t4381: F, t4397: F, t6459: F, t6477: F, t8319: F, t8328: F, t8332: F, t8337: F) -> (F, F) {
    let t27727 = t21524 * t27726;
    let t27730 = t5675 * t27720;
    let t27731 = t21530 * t27730;
    let t27739 = 0.23987718288236422951e-1 * t6459 * t6477 + 0.10794473229706390328e0 * t1580 * t27702 + 0.89953943580886586067e-2 * t27706 * t1583 - 0.59969295720591057377e-2 * t27710 - 0.31983624384315230603e-1 * t4381 * t8319 + 0.11993859144118211476e-1 * t4397 * t8319 - 0.17990788716177317213e-1 * t4397 * t8328 - 0.23987718288236422951e-1 * t4381 * t8332 - 0.23987718288236422951e-1 * t21500 * t27722 + 0.35981577432354634426e-1 * t21500 * t27727 + 0.35981577432354634426e-1 * t21500 * t27731 - 0.59969295720591057377e-2 * t21511 - 0.14392630972941853771e0 * t1572 * t8337 - t21515 - t21519 + 0.79959060960788076504e-2 * t21537 - t21555 - t21558 - t21567 - 0.59969295720591057378e-2 * t14940;
    (t27730, t27739)
}
