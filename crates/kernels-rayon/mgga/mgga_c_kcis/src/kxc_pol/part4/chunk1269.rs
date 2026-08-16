//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1269/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1269(t1330: f64, t16082: f64, t26: f64, t1324: f64, t16194: f64, t494: f64, t531: f64, t250: f64, t3106: f64, t11608: f64, t11609: f64, t16195: f64, t16198: f64, t16201: f64, t16204: f64, t16207: f64, t16210: f64) -> (f64, f64, f64, f64) {
    let t16212 = t1330 * t16082;
    let t16213 = t26 * t16212;
    let t16215 = t1324 * t16194;
    let t16217 = t494 * t531;
    let t16219 = t250 * t3106 * t16217;
    let t16221 = 0.1898925e1_f64 * t16195 + 0.16431333333333333333e0_f64 * t16198 - 0.49293999999999999999e0_f64 * t16201 - 0.27385555555555555556e-1_f64 * t16204 - 0.36514074074074074075e-1_f64 * t16207 + 0.10954222222222222222e0_f64 * t16210 + 0.16431333333333333333e0_f64 * t16213 + 0.3071625e0_f64 * t16215 - t11608 - t11609 + 0.16431333333333333333e0_f64 * t16219;
    (t16213, t16215, t16219, t16221)
}
