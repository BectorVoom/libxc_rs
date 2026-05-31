//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 426/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk426<F: Float>(t1615: F, t1616: F, t1390: F, t1399: F, t1461: F, t1492: F, t1500: F, t1588: F, t1592: F, t626: F, t632: F) -> (F, F, F, F, F) {
    let t1617 = t1615 * t1616;
    let t1620 = F::cast_from(0.11607361111111111111e-2_f64) * t1390;
    let t1625 = t1588 * t626 - F::cast_from(0.66725e-1_f64) * t1592 * t1617 + t1620 + F::cast_from(0.11607361111111111111e-2_f64) * t1399 + F::cast_from(0.17411041666666666666e-2_f64) * t1461 - F::cast_from(0.17411041666666666666e-2_f64) * t1492 + F::cast_from(0.11607361111111111111e-2_f64) * t1500;
    let t1627 = t632 * t632;
    let t1628 = F::cast_from(1.0_f64) / t1627;
    (t1617, t1620, t1625, t1627, t1628)
}
