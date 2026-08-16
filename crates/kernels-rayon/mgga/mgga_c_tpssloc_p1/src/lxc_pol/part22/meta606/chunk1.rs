//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2132/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2132(t3039: f64, t4599: f64, t49850: f64, t10870: f64, t4644: f64, t10875: f64, t48569: f64, t10903: f64, t14507: f64, t14651: f64, t3069: f64, t4608: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50258 = t3039 * t49850 * t4599;
    let t50259 = t50258 / 4608.0_f64;
    let t50262 = t4644 * t10870;
    let t50263 = t50262 / 6912.0_f64;
    let t50265 = t48569 * t10875;
    let t50302 = t14507 * t10903;
    let t50324 = t14651 * t3069;
    let t50361 = t973 * t698 * t4608;
    (t50259, t50263, t50265, t50302, t50324, t50361)
}
