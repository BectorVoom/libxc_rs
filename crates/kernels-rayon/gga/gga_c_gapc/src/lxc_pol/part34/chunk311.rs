//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 311/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk311(t54: f64, t1243: f64, t372: f64, t1179: f64, t1184: f64, t1191: f64, t1206: f64, t1214: f64, t1218: f64, t1222: f64, t1229: f64, t1240: f64) -> (f64, f64, f64, f64) {
    let t1244 = t54 * t54;
    let t1245 = 1.0_f64 / t1244;
    let t1246 = t1243 * t1245;
    let t1248 = 0.17315755899375863299e2_f64 * t372 * t1246;
    let t1249 = -t1179 - t1184 - t1191 + t1206 + t1214 + t1218 + t1222 + t1229 - t1240 - t1248;
    (t1245, t1246, t1248, t1249)
}
