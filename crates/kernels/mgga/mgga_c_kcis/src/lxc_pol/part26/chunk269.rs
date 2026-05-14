//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 269/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk269<F: Float>(t1419: F, t1482: F, t542: F, t1102: F, t1360: F, t1470: F, t1474: F, t1479: F, t344: F, t486: F) -> (F, F, F) {
    let t1483 = t1482 * t1419;
    let t1484 = t542 * t1483;
    let t1489 = t1470 + 0.65704296666666666667e-3 * t1102 * t1474 + 0.1478346675e-2 * t344 * t1479 - 0.98556445e-3 * t344 * t1484 - 4.0 * t486 * t1360;
    (t1483, t1484, t1489)
}
