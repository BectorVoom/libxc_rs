//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 373/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk373<F: Float>(t1596: F, t1597: F, t1334: F, t1344: F, t1408: F, t1443: F, t1448: F, t1453: F, t1553: F, t1557: F, t548: F, t554: F) -> (F, F, F, F, F) {
    let t1598 = t1596 * t1597;
    let t1601 = 0.11607361111111111111e-2 * t1334;
    let t1607 = t1553 * t548 - 0.193e0 * t1557 * t1598 + t1601 + 0.11607361111111111111e-2 * t1344 + 0.17411041666666666666e-2 * t1408 - 0.17411041666666666666e-2 * t1443 - 0.46429444444444444443e-2 * t1448 + 0.11607361111111111111e-2 * t1453;
    let t1609 = t554 * t554;
    let t1610 = 1.0 / t1609;
    (t1598, t1601, t1607, t1609, t1610)
}
