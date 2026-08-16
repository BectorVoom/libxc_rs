//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1382/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1382(t16050: f64, t11409: f64, t11411: f64, t11413: f64, t11415: f64, t12791: f64, t16046: f64, t16048: f64, t16052: f64, t16057: f64, t16062: f64, t16067: f64, t16071: f64, t16075: f64, t16080: f64, t16084: f64, t16088: f64) -> f64 {
    let t17905 = 0.2283111111111111111e-1_f64 * t16050;
    let t17915 = -t12791 - 0.1522074074074074074e-1_f64 * t11409 + 0.38051851851851851851e-2_f64 * t11411 - 0.11415555555555555555e-1_f64 * t11413 + 0.57077777777777777777e-2_f64 * t11415 - 0.76103703703703703702e-2_f64 * t16046 + 0.76103703703703703701e-2_f64 * t16048 - t17905 - 0.1255711111111111111e0_f64 * t16052 - 0.19025925925925925925e-1_f64 * t16057 + 0.68493333333333333331e-1_f64 * t16062 + 0.45662222222222222221e-1_f64 * t16067 - 0.11415555555555555555e-1_f64 * t16071 - 0.10274e0_f64 * t16075 - 0.13698666666666666666e0_f64 * t16080 + 0.34246666666666666666e-1_f64 * t16084 + 0.34246666666666666666e-1_f64 * t16088;
    t17915
}
