//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1185/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1185(t1860: f64, t22489: f64, t7031: f64, t1864: f64, t67: f64, t835: f64, t22534: f64, t7032: f64, t23993: f64, t6486: f64, t2031: f64, t2032: f64, t22519: f64, t7026: f64, t7035: f64, t83699: f64, t83706: f64, t83710: f64, t83771: f64, t83835: f64, t83840: f64, t83846: f64) -> f64 {
    let t84270 = t1860 * t7031 * t22489;
    let t84280 = 1232.0_f64 / 81.0_f64 * t1860 * t835 * t67 * t1864;
    let t84283 = t22534 * t7032;
    let t84285 = t6486 * t23993;
    let t84287 = -2.0_f64 * t83835 * t2032 - 4.0_f64 * t22519 * t7035 - 5.0_f64 * t7026 * t83771 - 5.0_f64 * t7026 * t83840 - 5.0_f64 / 3.0_f64 * t7026 * t83846 - 8.0_f64 / 3.0_f64 * t84270 + t1860 * t2031 * t83706 / 3.0_f64 + t83710 * t2032 / 3.0_f64 - t84280 - 2.0_f64 * t83699 * t2032 + 16.0_f64 / 3.0_f64 * t84283 + 88.0_f64 / 9.0_f64 * t84285;
    t84287
}
