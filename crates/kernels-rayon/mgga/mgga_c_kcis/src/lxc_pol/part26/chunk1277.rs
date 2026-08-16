//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1277/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1277(t12844: f64, t27583: f64, t29582: f64, t18171: f64, t29574: f64, t27567: f64, t101868: f64, t101871: f64, t101875: f64, t101884: f64, t27636: f64, t28755: f64, t28767: f64, t28816: f64, t28853: f64, t6176: f64, t77762: f64, t7968: f64, t7978: f64, t99024: f64, t99248: f64, t99331: f64) -> (f64, f64) {
    let t101892 = t27583 * t12844 * t29582;
    let t101894 = t18171 * t29574;
    let t101895 = t27567 * t101894;
    let t101898 = -0.82448622685185185186e-4_f64 * t99248 * t28755 + 0.8237654320987654321e-3_f64 * t99331 * t28767 - 0.23214722222222222221e-2_f64 * t101868 + 0.19345601851851851852e-2_f64 * t101871 + 0.208515625e-2_f64 * t7978 * t101875 - 0.69505208333333333334e-3_f64 * t7978 * t6176 * t27636 * t77762 - 0.34752604166666666667e-3_f64 * t7978 * t101884 - 0.46377350260416666667e-4_f64 * t7968 * t101884 + 0.24734586805555555555e-3_f64 * t28853 * t28816 + 0.7722800925925925926e-4_f64 * t101892 + 0.10306077835648148148e-4_f64 * t101895 - 0.30918233506944444445e-4_f64 * t99024;
    (t101894, t101898)
}
