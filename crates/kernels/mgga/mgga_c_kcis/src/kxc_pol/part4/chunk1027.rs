//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1027/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1027<F: Float>(t12614: F, t1599: F, t1607: F, t3978: F, t1606: F, t25: F, t4458: F, t4354: F, t597: F, t592: F, t11407: F, t11481: F) -> (F, F, F, F, F, F, F) {
    let t12615 = t1599 * t12614;
    let t12617 = t3978 * t1607;
    let t12650 = t1606 * t1606;
    let t12651 = F::new(1.0) / t12650;
    let t12663 = t25 * t4458;
    let t12664 = t1599 * t12663;
    let t12688 = F::new(1.0) / t4354 / t597;
    let t12689 = t592 * t12688;
    let t12717 = F::new(0.16068111111111111111e1) * t11407;
    let t12718 = F::new(0.46308888888888888888e0) * t11481;
    (t12615, t12617, t12651, t12664, t12689, t12717, t12718)
}
