//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 912/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk912<F: Float>(t16103: F, t5546: F, t16120: F, t5578: F, t1334: F, t6954: F, t3899: F, t1907: F, t5573: F, t3861: F, t6989: F, t11581: F) -> (F, F, F, F, F) {
    let t21351 = F::new(4.0) * t16103 * t5546;
    let t21353 = F::new(0.32163648644302209644e2) * t16120 * t5578;
    let t21354 = t6954 * t1334;
    let t21356 = F::new(6.0) * t3899 * t21354;
    let t21357 = t1907 * t5573;
    let t21359 = F::new(4.0) * t3861 * t21357;
    let t21360 = t6989 * t1334;
    let t21362 = F::new(0.96490945932906628932e2) * t11581 * t21360;
    (t21351, t21353, t21356, t21359, t21362)
}
