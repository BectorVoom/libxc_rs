//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 634/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk634<F: Float>(t556: F, t622: F, t3793: F, t1559: F, t1563: F, t1562: F, t597: F, t592: F, t3879: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4301 = F::new(1.0) / t556;
    let t4313 = t622 * t622;
    let t4314 = F::new(1.0) / t4313;
    let t4318 = F::new(0.22831111111111111111e-1) * t3793;
    let t4326 = t1559 * t1563;
    let t4329 = t1562 * t597;
    let t4330 = F::new(1.0) / t4329;
    let t4331 = t592 * t4330;
    let t4338 = F::new(0.68863333333333333333e0) * t3793;
    let t4345 = F::new(0.17365833333333333333e0) * t3879;
    let t4354 = t1562 * t1562;
    let t4355 = F::new(1.0) / t4354;
    let t4356 = t592 * t4355;
    (t4301, t4313, t4314, t4318, t4326, t4330, t4331, t4338, t4345, t4354, t4355, t4356)
}
