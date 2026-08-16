//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 296/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk296(t155: f64, t462: f64, t153: f64, t1233: f64, t1236: f64, t1240: f64, t1227: f64, t145: f64, t458: f64, t465: f64, t1230: f64, t1237: f64, t1242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1246 = 1.0_f64 / t462 / t155;
    let t1247 = t153 * t1246;
    let t1248 = t1247 * t1233;
    let t1250 = t1236 * t1240 * pi;
    let t1254 = t1227 * t145 * t458;
    let t1255 = t465 * t1254;
    let t1257 = 63.0_f64 / 256.0_f64 * t1230 - 49.0_f64 / 8192.0_f64 * t1237 * t1242 + 49.0_f64 / 24576.0_f64 * t1248 * t1250 - 21.0_f64 / 256.0_f64 * t1255;
    (t1246, t1247, t1250, t1254, t1255, t1257)
}
