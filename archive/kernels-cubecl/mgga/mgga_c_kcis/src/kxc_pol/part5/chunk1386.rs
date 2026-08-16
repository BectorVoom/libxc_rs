//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1386/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1386<F: Float>(t2080: F, t6097: F, t1571: F, t7463: F, t12689: F, t17797: F, t17834: F, t21172: F, t21174: F, t21176: F, t21178: F, t21180: F, t21293: F, t21295: F, t22833: F, t4331: F, t4356: F, t6080: F, t6102: F) -> F {
    let t22836 = t2080 * t6097;
    let t22839 = t7463 * t1571;
    let t22842 = -t21172 - t21174 - t21176 + t21178 - t21180 - t21293 - t21295 - F::cast_from(4.0_f64) * t17834 * t6080 + F::cast_from(0.64329366355741395948e2_f64) * t17797 * t6102 + F::cast_from(6.0_f64) * t4356 * t22833 - F::cast_from(4.0_f64) * t4331 * t22836 - F::cast_from(0.19298809906722418785e3_f64) * t12689 * t22839;
    t22842
}
