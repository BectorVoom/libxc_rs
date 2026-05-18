//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1406/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1406<F: Float>(t609: F, t4425: F, t7421: F, t1599: F, t6141: F, t6148: F, t23024: F, t1608: F, t286: F, t25: F, t7493: F, t7430: F, t6168: F) -> (F, F, F, F, F, F) {
    let t614 = F::new(0.0) < t609;
    let t23191 = t4425 * t7421;
    let t23192 = t1599 * t23191;
    let t23194 = t6141 * t6148;
    let t23198 = piecewise3::<f64>(t614, t23024, -t23024);
    let t23199 = t1608 * t23198;
    let t23200 = t286 * t23199;
    let t23207 = t25 * t7493;
    let t23208 = t1599 * t23207;
    let t23210 = t25 * t7430;
    let t23211 = t1599 * t23210;
    let t23213 = t6141 * t6168;
    (t23192, t23194, t23200, t23208, t23211, t23213)
}
