//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 319/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk319(t1122: f64, t1145: f64, t141: f64, t1124: f64, t1135: f64, t1137: f64, t1140: f64, t1144: f64, t421: f64) -> (f64, f64, f64, f64) {
    let t1146 = t1145 * t1122;
    let t1147 = t141 * t1146;
    let t1149 = 0.1898925e1_f64 * t1135 - t1137 + 0.29896666666666666667e0_f64 * t1124 + 0.3071625e0_f64 * t1140 - t1144 + 0.82156666666666666667e-1_f64 * t1147;
    let t1150 = 1.0_f64 / t421;
    (t1146, t1147, t1149, t1150)
}
