//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1258/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1258(t8124: f64, t25836: f64, t2602: f64, t7274: f64, t930: f64, t2704: f64, t7882: f64, t11518: f64, t19: f64, t23951: f64, t24569: f64, t25087: f64, t25595: f64, t2596: f64, t25969: f64, t25970: f64, t25972: f64, t25975: f64, t25977: f64, t25979: f64, t2643: f64, t2668: f64, t2708: f64, t323: f64, t7867: f64, t8058: f64, t894: f64, t953: f64, sigma0: f64) -> f64 {
    let t25981 = t8124 * sigma0;
    let t25982 = t25981 * t25836;
    let t25996 = t930 * t7274 * t2602;
    let t26000 = t2704 * t7882;
    let t26004 = -0.1209136907000740735e0_f64 * t953 * t25087 - t25969 - 0.93770531639908660928e4_f64 * t25970 + 0.46885265819954330464e4_f64 * t25972 + 0.16829779668897917239e1_f64 * t25975 + 0.80782942410710002747e1_f64 * t25977 - 0.12117441361606500412e2_f64 * t25979 + 0.18014732272771396904e7_f64 * t25982 * t323 * t24569 * t19 + 0.33587136305576131525e-1_f64 * t953 * t894 * t2596 * t23951 + 0.61944912485988186947e2_f64 * t2668 * t11518 * t25595 * t2643 + 0.7727254657590006982e-1_f64 * t25996 - 0.12475836244235246496e3_f64 * t2708 * t8058 + 0.42991534471137448352e0_f64 * t26000 - 0.64487301706706172529e0_f64 * t2704 * t7867;
    t26004
}
