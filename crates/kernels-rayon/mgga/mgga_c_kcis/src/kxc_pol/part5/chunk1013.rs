//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1013/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1013(t4682: f64, t930: f64, t1666: f64, t2985: f64, t4711: f64, t659: f64, t4708: f64, t13714: f64, t1676: f64, t2331: f64, t22: f64, t4864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13867 = t4682 * t930;
    let t13872 = t1666 * t2985;
    let t13908 = t659 * t4711;
    let t13909 = 0.21908444444444444444e0_f64 * t13908;
    let t13912 = t659 * t4708;
    let t13939 = 0.39862222222222222222e0_f64 * t13714;
    let t13945 = t2331 * t1676;
    let t13948 = t22 * t4864;
    (t13867, t13872, t13908, t13909, t13912, t13939, t13945, t13948)
}
