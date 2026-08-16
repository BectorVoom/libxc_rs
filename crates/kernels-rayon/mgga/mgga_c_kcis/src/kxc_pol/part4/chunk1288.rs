//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1288/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1288(t16048: f64, t16046: f64, t16050: f64, t16057: f64, t16067: f64, t16071: f64, t16080: f64, t16084: f64, t16135: f64, t16137: f64, t16168: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t11746: f64, t16052: f64, t16062: f64, t16075: f64, t16088: f64, t16195: f64) -> f64 {
    let t16523 = 0.18344444444444444444e-2_f64 * t16048;
    let t16529 = 0.14865e-1_f64 * t16168 - 0.1982e-1_f64 * t16135 - 0.991e-2_f64 * t16137 - 0.18344444444444444444e-2_f64 * t16046 - 0.55033333333333333333e-2_f64 * t16050 + t16523 - 0.27516666666666666667e-2_f64 * t16071 - 0.45861111111111111112e-2_f64 * t16057 + 0.11006666666666666667e-1_f64 * t16067 + 0.8255e-2_f64 * t16084 - 0.3302e-1_f64 * t16080;
    let t16530 = 0.1651e-1_f64 * t16062 - 0.30268333333333333334e-1_f64 * t16052 + 0.8255e-2_f64 * t16088 + 0.1982e-1_f64 * t16195 - 0.36688888888888888888e-2_f64 * t11409 + 0.13758333333333333333e-2_f64 * t11415 + 0.9172222222222222222e-3_f64 * t11411 - 0.24765e-1_f64 * t16075 - 0.27516666666666666666e-2_f64 * t11413 - t11746 + t16529;
    t16530
}
