//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1045/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1045<F: Float>(t524: F, t1580: F, t27777: F, t4419: F, t8399: F, t535: F, t8336: F, t27682: F, t1589: F, t1586: F, t1568: F, t1572: F, t1593: F, t2318: F, t2328: F, t27744: F, t27749: F, t27756: F, t27761: F, t27766: F, t27770: F, t27774: F, t4381: F, t6450: F, t6459: F, t6486: F, t6490: F, t6583: F, t8308: F, t8324: F, t8337: F, t8400: F) -> (F, F) {
    let t536 = 0.0 < t524;
    let t27778 = t1580 * t27777;
    let t27790 = t4419 * t8399;
    let t27791 = t535 * t27790;
    let t27795 = t4419 * t8336;
    let t27796 = t535 * t27795;
    let t27801 = piecewise3(t536, t27682, -t27682);
    let t27802 = t1589 * t27801;
    let t27803 = t1586 * t27802;
    let t27806 = 0.35981577432354634426e-1 * t6459 * t6490 + 0.5397236614853195164e-1 * t1580 * t27744 - 0.16191709844559585492e0 * t1580 * t27749 - 0.35981577432354634426e-1 * t6459 * t6486 - 0.89953943580886586067e-2 * t1580 * t27756 + 0.17990788716177317213e-1 * t1580 * t27761 - 0.89953943580886586067e-2 * t1580 * t27766 - 0.17990788716177317213e-1 * t1580 * t27770 + 0.11993859144118211476e-1 * t1580 * t27774 - 0.59969295720591057377e-2 * t27778 + 0.47975436576472845901e-1 * t4381 * t8324 + 0.5397236614853195164e-1 * t1568 * t8337 - 0.5397236614853195164e-1 * t6450 * t2328 - 0.5397236614853195164e-1 * t2318 * t6583 - 0.2698618307426597582e-1 * t8308 * t1593 - 0.89953943580886586067e-2 * t27791 + 0.71963154864709268853e-1 * t1572 * t8400 + 0.17990788716177317213e-1 * t27796 - 0.2698618307426597582e-1 * t1568 * t8400 - 0.2698618307426597582e-1 * t535 * t27803;
    (t27801, t27806)
}
