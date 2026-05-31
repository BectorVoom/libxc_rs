//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 967/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk967<F: Float>(t1571: F, t17815: F, t4350: F, t6101: F, t12732: F, t2079: F, t4332: F, t12689: F, t12730: F, t12761: F, t12767: F, t12772: F, t1577: F, t17792: F, t17797: F, t17806: F, t17809: F, t17812: F, t2080: F, t4331: F, t4351: F, t4356: F, t4359: F, t4363: F, t6075: F, t6080: F, t6102: F, t6114: F) -> F {
    let t17816 = t17815 * t1571;
    let t17819 = t6101 * t4350;
    let t17822 = t2079 * t12732;
    let t17823 = t17822 * t4332;
    let t17826 = F::cast_from(0.11696446794910408142e1_f64) * t4363 * t6114 + F::cast_from(0.58482233974552040708e0_f64) * t1577 * t17792 + F::cast_from(1.0_f64) * t6075 * t4351 + F::cast_from(0.32164683177870697974e2_f64) * t17797 * t4359 + F::cast_from(1.0_f64) * t12767 * t2080 - F::cast_from(4.0_f64) * t12761 * t6080 + F::cast_from(0.64329366355741395948e2_f64) * t12772 * t6102 - F::cast_from(4.0_f64) * t4331 * t17806 - F::cast_from(2.0_f64) * t4331 * t17809 - F::cast_from(0.19298809906722418785e3_f64) * t12689 * t17812 + F::cast_from(0.64329366355741395948e2_f64) * t4356 * t17816 + F::cast_from(0.32164683177870697974e2_f64) * t4356 * t17819 + F::cast_from(0.20691336878655965246e4_f64) * t12730 * t17823;
    t17826
}
