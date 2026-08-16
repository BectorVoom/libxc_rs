//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1143/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1143(t121206: f64, t121232: f64, t121177: f64, t1385: f64, t240: f64, t27: f64, t119967: f64, t121204: f64, t13847: f64, t1399: f64, t121210: f64, t2453: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121233 = t121232 * t121206;
    let t121234 = 0.150583822711895824e-3_f64 * t121233;
    let t121235 = t121232 * t121177;
    let t121245 = t1385 * t27 * t240;
    let t121246 = t119967 * t121245;
    let t121248 = t13847 * t121204 * t1399;
    let t121249 = t121246 * t121248;
    let t121272 = t2453 * t8705 * t121210;
    (t121234, t121235, t121245, t121246, t121248, t121249, t121272)
}
