//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 410/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk410(t2131: f64, t231: f64, t1216: f64, t725: f64, t1227: f64, t728: f64, t1238: f64, t1246: f64, t1179: f64, t1184: f64, t1191: f64, t1206: f64, t1214: f64, t2042: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2132 = t231 * t2131;
    let t2134 = 0.24415406715670879921e-3_f64 * t725 * t1216;
    let t2136 = 0.11696446794910408142e1_f64 * t728 * t1227;
    let t2138 = 0.58482233974552040708e0_f64 * t728 * t1238;
    let t2140 = 0.17315755899375863299e2_f64 * t728 * t1246;
    let t2141 = -t1179 - t1184 - t1191 + t1206 + t1214 + t2132 + t2134 + t2042 + t2136 - t2138 - t2140;
    (t2132, t2134, t2136, t2138, t2140, t2141)
}
