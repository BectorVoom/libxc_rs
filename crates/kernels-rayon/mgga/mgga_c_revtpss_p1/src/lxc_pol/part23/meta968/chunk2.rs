//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3268/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3268(t162: f64, t85950: f64, t85968: f64, t187: f64, t48297: f64, t48304: f64, t48306: f64, t47093: f64, t39989: f64, t47084: f64, t47086: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t48300: f64, t48303: f64, t85928: f64, t85930: f64, t85932: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85970 = (t85950 + t85968) * t162;
    let t85972 = 0.19751673498613801407e-1_f64 * t85970 * t187;
    let t85973 = 0.30762056574649219972e4_f64 * t48297;
    let t85974 = 0.48796115851357829289e-1_f64 * t48304;
    let t85975 = 0.14447919941302971323e1_f64 * t48306;
    let t85976 = 0.10389515463408878255e3_f64 * t47093;
    let t85977 = -t85928 + t85930 - t85932 + t85972 - t85973 - t47084 - t48300 + t48303 + t85974 + t85975 - t39989 - t47086 + t47088 + t47092 + t85976 - t47096 - t47098;
    (t85970, t85972, t85973, t85974, t85975, t85976, t85977)
}
