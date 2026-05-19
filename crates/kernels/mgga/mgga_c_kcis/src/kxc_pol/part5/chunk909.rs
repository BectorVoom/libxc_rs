//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 909/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk909<F: Float>(t12: F, t3: F, t160: F, t2326: F, t8581: F, t656: F, t8590: F, t4620: F, t4714: F, t8594: F, t8596: F, t8598: F) -> (F, F, F, F) {
    let t8689 = F::new(1.0)/pow_3_2::<F>(t12);
    let t8690 = t8689 * t3;
    let t8691 = t8690 * t160;
    let t8693 = t2326 * t8581;
    let t8695 = t656 * t8590;
    let t8698 = -F::cast_from(0.34523333333333333333e1_f64) * t8594 + F::cast_from(0.23015555555555555556e1_f64) * t8596 - F::cast_from(0.26851481481481481482e1_f64) * t8598 - F::cast_from(0.93932222222222222223e0_f64) * t4620 + F::new(0.73355e-1) * t8691 - F::new(0.14671e0) * t8693 - F::cast_from(0.17116166666666666667e0_f64) * t8695 - F::cast_from(0.36793333333333333333e0_f64) * t4714;
    (t8691, t8693, t8695, t8698)
}
