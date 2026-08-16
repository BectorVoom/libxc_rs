//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 253/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk253(t1143: f64, t240: f64, t462: f64, t1122: f64, t141: f64, t1124: f64, t1135: f64, t1137: f64, t1140: f64, t421: f64, t1131: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1144 = 0.82156666666666666667e-1_f64 * t1143;
    let t1145 = t240 * t462;
    let t1146 = t1145 * t1122;
    let t1147 = t141 * t1146;
    let t1149 = 0.1898925e1_f64 * t1135 - t1137 + 0.29896666666666666667e0_f64 * t1124 + 0.3071625e0_f64 * t1140 - t1144 + 0.82156666666666666667e-1_f64 * t1147;
    let t1150 = 1.0_f64 / t421;
    let t1151 = t1149 * t1150;
    let t1153 = 1.0_f64 * t1131 * t1151;
    let t1154 = 0.17123333333333333333e-1_f64 * t1118;
    (t1144, t1145, t1146, t1147, t1149, t1150, t1151, t1153, t1154)
}
