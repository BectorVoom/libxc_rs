//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 751/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk751(t154: f64, t8750: f64, t808: f64, t152: f64, t8536: f64, t8538: f64, t143: f64, t8747: f64, t21: f64, t2469: f64, t2553: f64, t2459: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9061 = t154 * t8750;
    let t9062 = t808 * t9061;
    let t9064 = t152 * t8536;
    let t9065 = t154 * t8538;
    let t9066 = t9064 * t9065;
    let t9070 = t8747 * t143;
    let t9074 = t2553 * t21 * t2469;
    let t9077 = t88 * t2459;
    (t9062, t9064, t9066, t9070, t9074, t9077)
}
