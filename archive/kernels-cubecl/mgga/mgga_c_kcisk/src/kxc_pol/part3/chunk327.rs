//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 327/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk327<F: Float>(t1344: F, t1408: F, t1443: F, t1448: F, t1453: F, t1553: F, t1557: F, t1598: F, t1601: F, t548: F, t554: F, t551: F) -> (F, F, F, F) {
    let t1607 = t1553 * t548 - F::cast_from(0.193e0_f64) * t1557 * t1598 + t1601 + F::cast_from(0.11607361111111111111e-2_f64) * t1344 + F::cast_from(0.17411041666666666666e-2_f64) * t1408 - F::cast_from(0.17411041666666666666e-2_f64) * t1443 - F::cast_from(0.46429444444444444443e-2_f64) * t1448 + F::cast_from(0.11607361111111111111e-2_f64) * t1453;
    let t1609 = t554 * t554;
    let t1610 = F::cast_from(1.0_f64) / t1609;
    let t1611 = t551 * t1610;
    (t1607, t1609, t1610, t1611)
}
