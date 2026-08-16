//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 571/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk571(t2943: f64, t2944: f64, t2917: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64) -> (f64, f64, f64) {
    let t2945 = t2943 * t2944;
    let t2947 = 4.0_f64 / 9.0_f64 * t2917;
    let t2952 = t2947 + 2.0_f64 / 9.0_f64 * t2919 - 2.0_f64 / 9.0_f64 * t2922 + 2.0_f64 / 3.0_f64 * t2925 - t2928 / 3.0_f64;
    (t2945, t2947, t2952)
}
