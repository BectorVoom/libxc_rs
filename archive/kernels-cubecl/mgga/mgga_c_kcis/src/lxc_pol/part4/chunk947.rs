//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 947/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk947<F: Float>(t161: F, t9175: F, t2491: F, t823: F, t2490: F, t2584: F, t754: F, t809: F, t9062: F, t9066: F, t9150: F, t9152: F, t9155: F, t9158: F, t9163: F, t9166: F, t9168: F, t9170: F, t9173: F) -> (F, F, F, F) {
    let t9176 = t9175 * t161;
    let t9178 = t823 * t2491;
    let t9179 = t2490 * t9178;
    let t9181 = t2584 * t754;
    let t9182 = t9181 * t809;
    let t9184 = -F::cast_from(0.1875e0_f64) * t9062 - F::cast_from(0.1125e1_f64) * t9066 + F::cast_from(0.1875e0_f64) * t9150 - F::cast_from(0.5625e0_f64) * t9152 + F::cast_from(0.2428125e0_f64) * t9155 + F::cast_from(0.4046875e-1_f64) * t9158 + F::cast_from(0.485625e1_f64) * t9163 - F::cast_from(0.225e1_f64) * t9166 - F::cast_from(0.1125e1_f64) * t9168 + F::cast_from(0.12140625e0_f64) * t9170 + F::cast_from(0.1125e1_f64) * t9173 - F::cast_from(0.4046875e-1_f64) * t9176 + F::cast_from(0.97125e0_f64) * t9179 - F::cast_from(0.5625e0_f64) * t9182;
    (t9176, t9179, t9182, t9184)
}
