//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 362/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk362(t1028: f64, t1221: f64, t914: f64, t1034: f64, t1059: f64, t1099: f64, t1101: f64, t1106: f64, t1186: f64, t1188: f64, t1210: f64, t1216: f64, t1220: f64, t277: f64, t498: f64, t95: f64) -> (f64, f64, f64) {
    let t1222 = t1221 * t1028;
    let t1223 = t914 * t1222;
    let t1226 = -t1034 + t1059 + t1099 + t1101 - t1106 + 0.25844881434903430496e-2_f64 * t95 * t277 * t1186 * t1188 + t1210 * t498 / 2.0_f64 + t1216 + t1220 * t1223 / 6.0_f64;
    (t1222, t1223, t1226)
}
