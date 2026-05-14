//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 321/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk321<F: Float>(t1589: F, t1591: F, t1586: F, t1568: F, t1572: F, t1578: F, t1580: F, t1583: F, t535: F, t541: F, t544: F, t1334: F, t1344: F, t1408: F, t1443: F, t1448: F, t1453: F, t1553: F, t1557: F, t548: F) -> (F, F, F, F, F, F) {
    let t1592 = t1589 * t1591;
    let t1593 = t1586 * t1592;
    let t1596 = 0.2698618307426597582e-1 * t1568 * t541 - 0.71963154864709268853e-1 * t1572 * t541 + t1578 + 0.89953943580886586067e-2 * t1580 * t1583 - 0.2698618307426597582e-1 * t535 * t1593;
    let t1597 = 1.0 / t544;
    let t1598 = t1596 * t1597;
    let t1601 = 0.11607361111111111111e-2 * t1334;
    let t1607 = t1553 * t548 - 0.193e0 * t1557 * t1598 + t1601 + 0.11607361111111111111e-2 * t1344 + 0.17411041666666666666e-2 * t1408 - 0.17411041666666666666e-2 * t1443 - 0.46429444444444444443e-2 * t1448 + 0.11607361111111111111e-2 * t1453;
    (t1592, t1593, t1596, t1597, t1598, t1607)
}
