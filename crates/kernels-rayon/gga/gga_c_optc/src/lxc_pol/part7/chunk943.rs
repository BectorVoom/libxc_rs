//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 943/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk943(t123: f64, t3107: f64, t1028: f64, t8968: f64, t1111: f64, t1121: f64, t1133: f64, t8531: f64, t8534: f64, t8539: f64, t8543: f64, t8548: f64, t8908: f64, t8913: f64, t8918: f64, t8921: f64, t8924: f64, t8928: f64, t8930: f64, t8933: f64, t8937: f64, t8941: f64, t8944: f64, t8947: f64, t8953: f64, t8957: f64, t8960: f64, t8963: f64, t8966: f64, t8970: f64, t8973: f64) -> (f64, f64, f64, f64) {
    let t8974 = t3107 * t123;
    let t8975 = t8974 * t1028;
    let t8976 = t8968 * t8975;
    let t8979 = t8531 - t1111 * t8534 / 36.0_f64 + t1111 * t8539 / 48.0_f64 - t8543 / 144.0_f64 + t8548 + 0.35500316489081544176e-1_f64 * t1121 * t8908 + 0.10629507243271336419e5_f64 * t8913 * t8918 - 0.10629507243271336419e5_f64 * t8921 * t8924 + 0.71000632978163088351e-1_f64 * t8928 - t1111 * t8930 / 48.0_f64 + t1111 * t8933 / 72.0_f64 + t1111 * t8937 / 288.0_f64 - t8941 / 432.0_f64 + t8944 / 288.0_f64 + 0.60369177012421929545e-2_f64 * t8947 + 0.80492236016562572728e-2_f64 * t1133 * t8953 + 0.18110753103726578864e-2_f64 * t1133 * t8957 + 0.17715845405452227366e4_f64 * t8960 * t8963 - 0.91572784804598301689e1_f64 * t8966 * t8970 + 0.18314556960919660338e2_f64 * t8973 * t8976;
    (t8974, t8975, t8976, t8979)
}
