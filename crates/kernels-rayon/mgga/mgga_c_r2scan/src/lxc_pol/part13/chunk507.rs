//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 507/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk507(t1267: f64, t2104: f64, t2279: f64, t2281: f64, t2289: f64, t2293: f64, t2295: f64, t2302: f64, t2304: f64, t269: f64, t550: f64, t864: f64, t870: f64) -> f64 {
    let t2312 = 2.0_f64 * t2279 * t864 - 1.0_f64 * t2281 * t864 + 1.0_f64 * t2289 * t864 + 0.2845018947250181111e-1_f64 * t2293 * t2295 - 0.20235332025531322028e-2_f64 * t2302 * t2104 * t269 * t2304 + 0.52158680699586653702e-1_f64 * t870 * t550 * t1267;
    t2312
}
