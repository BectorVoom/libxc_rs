//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1292/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1292(t16103: f64, t5546: f64, t16120: f64, t5578: f64, t1334: f64, t6954: f64, t3899: f64, t1907: f64, t5573: f64, t3861: f64, t6989: f64, t11581: f64) -> (f64, f64, f64, f64, f64) {
    let t21351 = 4.0_f64 * t16103 * t5546;
    let t21353 = 0.32163648644302209644e2_f64 * t16120 * t5578;
    let t21354 = t6954 * t1334;
    let t21356 = 6.0_f64 * t3899 * t21354;
    let t21357 = t1907 * t5573;
    let t21359 = 4.0_f64 * t3861 * t21357;
    let t21360 = t6989 * t1334;
    let t21362 = 0.96490945932906628932e2_f64 * t11581 * t21360;
    (t21351, t21353, t21356, t21359, t21362)
}
