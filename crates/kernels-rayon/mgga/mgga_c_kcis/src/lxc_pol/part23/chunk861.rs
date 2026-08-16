//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 861/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk861(t11543: f64, t5546: f64, t11576: f64, t5578: f64, t1334: f64, t5574: f64, t3861: f64, t1907: f64, t3893: f64, t3862: f64, t5577: f64, t11581: f64) -> (f64, f64, f64, f64, f64) {
    let t16251 = 4.0_f64 * t11543 * t5546;
    let t16253 = 0.32163648644302209644e2_f64 * t11576 * t5578;
    let t16254 = t5574 * t1334;
    let t16256 = 4.0_f64 * t3861 * t16254;
    let t16257 = t1907 * t3893;
    let t16259 = 2.0_f64 * t3861 * t16257;
    let t16260 = t5577 * t3862;
    let t16262 = 0.96490945932906628932e2_f64 * t11581 * t16260;
    (t16251, t16253, t16256, t16259, t16262)
}
