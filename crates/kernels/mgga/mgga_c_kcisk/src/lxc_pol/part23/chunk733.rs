//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 733/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk733<F: Float>(t1589: F, t6581: F, t1586: F, t1568: F, t1572: F, t1580: F, t2322: F, t2328: F, t4381: F, t535: F, t6474: F, t6477: F, t6482: F, t6486: F, t6490: F, t6498: F, t6502: F, t6507: F) -> (F, F, F) {
    let t6582 = t1589 * t6581;
    let t6583 = t1586 * t6582;
    let t6586 = -0.23987718288236422951e-1 * t4381 * t2322 + 0.29984647860295528689e-2 * t6474 + 0.11993859144118211476e-1 * t1580 * t6477 - 0.89953943580886586067e-2 * t1580 * t6482 - 0.17990788716177317213e-1 * t1580 * t6486 + 0.17990788716177317213e-1 * t1580 * t6490 - 0.2698618307426597582e-1 * t1568 * t2328 + 0.71963154864709268853e-1 * t1572 * t2328 - 0.89953943580886586067e-2 * t6498 - 0.89953943580886586067e-2 * t1580 * t6502 + 0.5397236614853195164e-1 * t1580 * t6507 - 0.2698618307426597582e-1 * t535 * t6583;
    (t6582, t6583, t6586)
}
