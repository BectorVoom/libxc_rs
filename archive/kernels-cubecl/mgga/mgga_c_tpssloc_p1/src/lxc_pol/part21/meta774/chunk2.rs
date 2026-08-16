//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2682/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2682<F: Float>(t157: F, t56323: F, t56347: F, t182: F, t1390: F, t20063: F, t54412: F, t39491: F, t12466: F, t1307: F, t16148: F, t3918: F, t39483: F, t39490: F, t39496: F, t5122: F, t5126: F, t56298: F, t56299: F, t6330: F) -> (F, F, F, F, F) {
    let t56349 = (t56323 + t56347) * t157;
    let t56351 = F::cast_from(0.19751673498613801407e-1_f64) * t56349 * t182;
    let t56358 = t20063 * t1390;
    let t56362 = F::cast_from(24.0_f64) * t54412;
    let t56363 = F::cast_from(0.11696447245269292414e1_f64) * t39491;
    let t56364 = F::cast_from(6.0_f64) * t12466 * t5126 * t6330 + F::cast_from(6.0_f64) * t1307 * t3918 * t56358 + F::cast_from(24.0_f64) * t16148 * t5122 * t5126 + t39483 - t39490 - t39496 + t56298 + t56299 + t56351 - t56362 + t56363;
    (t56349, t56351, t56362, t56363, t56364)
}
