//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1692/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1692(t10298: f64, t10301: f64, t10309: f64, t10310: f64, t10313: f64, t10410: f64, t2242: f64, t2247: f64, t2248: f64, t2315: f64, t45953: f64, t45955: f64, t45958: f64, t45963: f64, t45972: f64, t45973: f64, t45979: f64, t46034: f64, t46119: f64, t603: f64, t644: f64, t91: f64) -> f64 {
    let t46123 = t45953 * t91 - 16.0_f64 * t45955 * t644 + 120.0_f64 * t45958 * t2248 - 24.0_f64 * t10298 * t2315 - 480.0_f64 * t45963 * t10310 + 240.0_f64 * t10301 * t10313 - 16.0_f64 * t2242 * t10410 + 840.0_f64 * t45972 * t45973 - 720.0_f64 * t10309 * t2248 * t2315 + 60.0_f64 * t2247 * t45979 + 80.0_f64 * t2247 * t644 * t10410 - 4.0_f64 * t603 * (t46034 + t46119);
    t46123
}
