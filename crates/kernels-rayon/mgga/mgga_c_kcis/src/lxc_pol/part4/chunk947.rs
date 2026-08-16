//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 947/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk947(t161: f64, t9175: f64, t2491: f64, t823: f64, t2490: f64, t2584: f64, t754: f64, t809: f64, t9062: f64, t9066: f64, t9150: f64, t9152: f64, t9155: f64, t9158: f64, t9163: f64, t9166: f64, t9168: f64, t9170: f64, t9173: f64) -> (f64, f64, f64, f64) {
    let t9176 = t9175 * t161;
    let t9178 = t823 * t2491;
    let t9179 = t2490 * t9178;
    let t9181 = t2584 * t754;
    let t9182 = t9181 * t809;
    let t9184 = -0.1875e0_f64 * t9062 - 0.1125e1_f64 * t9066 + 0.1875e0_f64 * t9150 - 0.5625e0_f64 * t9152 + 0.2428125e0_f64 * t9155 + 0.4046875e-1_f64 * t9158 + 0.485625e1_f64 * t9163 - 0.225e1_f64 * t9166 - 0.1125e1_f64 * t9168 + 0.12140625e0_f64 * t9170 + 0.1125e1_f64 * t9173 - 0.4046875e-1_f64 * t9176 + 0.97125e0_f64 * t9179 - 0.5625e0_f64 * t9182;
    (t9176, t9179, t9182, t9184)
}
