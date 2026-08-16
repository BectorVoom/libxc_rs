//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1261/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1261(t11409: f64, t11411: f64, t11413: f64, t11415: f64, t11557: f64, t16046: f64, t16048: f64, t16051: f64, t16052: f64, t16057: f64, t16062: f64, t16067: f64, t16071: f64, t16075: f64, t16080: f64, t16084: f64, t16088: f64) -> f64 {
    let t16090 = -t11557 - 0.15829629629629629629e-1_f64 * t11409 + 0.39574074074074074073e-2_f64 * t11411 - 0.11872222222222222222e-1_f64 * t11413 + 0.5936111111111111111e-2_f64 * t11415 - 0.79148148148148148146e-2_f64 * t16046 + 0.79148148148148148146e-2_f64 * t16048 - t16051 - 0.13059444444444444444e0_f64 * t16052 - 0.19787037037037037037e-1_f64 * t16057 + 0.71233333333333333332e-1_f64 * t16062 + 0.47488888888888888888e-1_f64 * t16067 - 0.11872222222222222222e-1_f64 * t16071 - 0.10685e0_f64 * t16075 - 0.14246666666666666666e0_f64 * t16080 + 0.35616666666666666666e-1_f64 * t16084 + 0.35616666666666666666e-1_f64 * t16088;
    t16090
}
