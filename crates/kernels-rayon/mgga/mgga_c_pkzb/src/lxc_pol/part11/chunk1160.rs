//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1160/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1160(t10208: f64, t23213: f64, t3185: f64, t10204: f64, t3206: f64, t10088: f64, t6475: f64, t10093: f64, t926: f64, t10191: f64, t2099: f64, t918: f64) -> (f64, f64, f64, f64, f64) {
    let t28231 = t3185 * t23213 * t10208;
    let t28234 = t3206 * t23213 * t10204;
    let t28263 = t3185 * t6475 * t10088;
    let t28266 = t3185 * t926 * t10093;
    let t28283 = t918 * t2099 * t10191;
    (t28231, t28234, t28263, t28266, t28283)
}
