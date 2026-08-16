//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 964/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk964(t13296: f64, t1564: f64, t11359: f64, t2492: f64, t4752: f64, t3377: f64, t38181: f64, t1339: f64, t1445: f64, t1537: f64, t1562: f64, t2859: f64, t3358: f64, t44404: f64, t46257: f64, t46261: f64, t46264: f64, t46267: f64, t46271: f64, t46275: f64, t46283: f64, t46287: f64, t46289: f64, t46291: f64, t46294: f64, t46297: f64, t46299: f64, t46301: f64, t46303: f64, t475: f64, t590: f64) -> f64 {
    let t46304 = t1564 * t13296;
    let t46311 = 0.28600391961480341335e1_f64 * t11359 * t4752 * t2492;
    let t46316 = 0.10725146985555128001e1_f64 * t38181 * t3377;
    let t46317 = -t46257 - t46261 + t46264 - t46267 + t46271 + t46275 - 0.51123901271894332902e1_f64 * t1537 * t1339 * t44404 * t590 + t46283 - t46287 - t46289 - t46291 + t46294 + t46297 - t46299 + t46301 + t46303 - 0.69017266717057349418e1_f64 * t1562 * t1445 * t46304 * t475 + t46311 - 0.14300195980740170668e1_f64 * t2859 * t4752 * t3358 - t46316;
    t46317
}
